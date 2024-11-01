use std::mem::size_of;

use drillx::Solution;
use luckycoin_api::{
    consts::*,
    error::LuckycoinError,
    event::MineEvent,
    instruction::Mine,
    loaders::*,
    state::{Bus, Config, Proof},
};
use solana_program::{clock, program::set_return_data};
#[allow(deprecated)]
use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    entrypoint::ProgramResult,
    keccak::hashv,
    program_error::ProgramError,
    pubkey::Pubkey,
    sanitize::SanitizeError,
    serialize_utils::{read_pubkey, read_u16},
    slot_hashes::SlotHash,
    sysvar::{self, Sysvar},
};
use steel::*;

pub fn process_mine(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = Mine::try_from_bytes(data)?;
    let [
        signer, 
        bus_info, 
        config_info, 
        proof_info, 
        instructions_sysvar, 
        slot_hashes_sysvar
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys); 
    };
    load_signer(signer)?; 
    load_any_bus(bus_info, true)?; 
    load_config(config_info, false)?; 
    load_proof_with_miner(proof_info, signer.key, true)?; 
    load_sysvar(instructions_sysvar, sysvar::instructions::id())?; 
    load_sysvar(slot_hashes_sysvar, sysvar::slot_hashes::id())?; 

    // 认证证明账户
    authenticate(&instructions_sysvar.data.borrow(), proof_info.key)?;

    // 验证当前纪元是否有效，确保挖矿操作在合适的时间窗口内有效进行
    // 检查当前时间是否已超过上次重置时间加上设定的纪元持续时间
    let config_data = config_info.data.borrow();
    let config = Config::try_from_bytes(&config_data)?;
    let clock = Clock::get().or(Err(ProgramError::InvalidAccountData))?;
    if config.last_reset_at.saturating_add(EPOCH_DURATION).le(&clock.unix_timestamp){
        return Err(LuckycoinError::NeedsReset.into()); 
    }

    // 验证哈希摘要(解析出证明信息，创建解决方案并验证是否符合挑战条件)
    let mut proof_data = proof_info.data.borrow_mut();
    let proof = Proof::try_from_bytes_mut(&mut proof_data)?;
    let solution = Solution::new(args.digest, args.nonce);
    if !solution.is_valid(&proof.challenge) {
        return Err(LuckycoinError::HashInvalid.into()); 
    }

    // 拒绝垃圾邮件事务
    let t: i64 = clock.unix_timestamp; 
    let t_target = proof.last_hash_at.saturating_add(ONE_MINUTE); 
    let t_spam = t_target.saturating_sub(TOLERANCE); 
    if t.lt(&t_spam) {
        return Err(LuckycoinError::Spam.into()); 
    }

    // 验证哈希满足最低难度
    let hash = solution.to_hash(); 
    let difficulty = hash.difficulty(); 
    if difficulty.lt(&(config.min_difficulty as u32)) {
        return Err(LuckycoinError::HashTooEasy.into()); 
    }

    // 规范化难度并计算奖励金额
    let normalized_difficulty = difficulty.checked_sub(config.min_difficulty as u32).unwrap();
    let mut reward = config
        .base_reward_rate
        .checked_mul(2u64.checked_pow(normalized_difficulty).unwrap())
        .unwrap();

    // 应用质押乘数
    let mut bus_data = bus_info.data.borrow_mut();
    let bus = Bus::try_from_bytes_mut(&mut bus_data)?;
    if proof.balance.gt(&0) && proof.last_stake_at.saturating_add(ONE_MINUTE).lt(&t) {
        // 计算质押奖励
        if config.top_balance.gt(&0) {
            let staking_reward = (reward as u128)
                .checked_mul(proof.balance.min(config.top_balance) as u128)
                .unwrap()
                .checked_div(config.top_balance as u128)
                .unwrap() as u64;
            reward = reward.checked_add(staking_reward).unwrap();
        }

        // 更新总线质押跟踪器
        if proof.balance.gt(&bus.top_balance) {
            bus.top_balance = proof.balance; // 更新最高余额
        }
    }

    // 应用活跃度惩罚
    //
    // The liveness penalty exists to ensure there is no "invisible" hashpower on the network. It
    // should not be possible to spend ~1 hour on a given challenge and submit a hash with a large
    // difficulty value to earn an outsized reward.
    //
    // The penalty works by halving the reward amount for every minute late the solution has been submitted.
    // This ultimately drives the reward to zero given enough time (10-20 minutes).
    let t_liveness = t_target.saturating_add(TOLERANCE);
    if t.gt(&t_liveness) {
        // 每分钟迟交奖励减半
        let tardiness = t.saturating_sub(t_target) as u64;
        let halvings = tardiness.saturating_div(ONE_MINUTE as u64);
        if halvings.gt(&0) {
            reward = reward.saturating_div(2u64.saturating_pow(halvings as u32));
        }

        // 余秒线性衰减
        let remainder_secs = tardiness.saturating_sub(halvings.saturating_mul(ONE_MINUTE as u64));
        if remainder_secs.gt(&0) && reward.gt(&0) {
            let penalty = reward
                .saturating_div(2)
                .saturating_mul(remainder_secs)
                .saturating_div(ONE_MINUTE as u64);
            reward = reward.saturating_sub(penalty);
        }
    }

    // 限制支付金额到总线剩余部分
    let reward_actual = reward.min(bus.rewards).min(ONE_ORE); // 实际奖励

    // 更新余额
    //
    // We track the theoretical rewards that would have been paid out ignoring the bus limit, so the
    // base reward rate will be updated to account for the real hashpower on the network.
    bus.theoretical_rewards = bus.theoretical_rewards.checked_add(reward).unwrap();
    bus.rewards = bus.rewards.checked_sub(reward_actual).unwrap();
    proof.balance = proof.balance.checked_add(reward_actual).unwrap();

    // 将最近的插槽哈希哈希到下一个挑战中
    // Hash a recent slot hash into the next challenge to prevent pre-mining attacks.
    //
    // The slot hashes are unpredictable values. By seeding the next challenge with the most recent slot hash,
    // miners are forced to submit their current solution before they can begin mining for the next.
    proof.last_hash = hash.h;
    proof.challenge = hashv(&[
        hash.h.as_slice(),
        &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()],
    ])
        .0;

    // 跟新时间跟踪器
    proof.last_hash_at = t.max(t_target);

    // 更新生命周期统计
    proof.total_hashes = proof.total_hashes.saturating_add(1);
    proof.total_rewards = proof.total_rewards.saturating_add(reward);

    // 记录挖矿奖励
    //
    // This data can be used by off-chain indexers to display mining stats.
    set_return_data(
        MineEvent {
            difficulty: difficulty as u64,
            reward: reward_actual,
            timing: t.saturating_sub(t_liveness),
        }.to_bytes(),
    );

    Ok(())
}

/// 认证证明账户，以防止 Sybil 攻击。
fn authenticate(data: &[u8], proof_address: &Pubkey) -> ProgramResult {
    if let Ok(Some(auth_address)) = parse_auth_address(data) {
        if proof_address.ne(&auth_address) {
            return Err(LuckycoinError::AuthFailed.into()); //返回错误，认证失败
        }
    } else {
        return Err(LuckycoinError::AuthFailed.into()); // 发挥错误，认证失败
    }
    Ok(())
}

/// 使用事务内省来解析认证的公钥
fn parse_auth_address(data: &[u8]) -> Result<Option<Pubkey>, SanitizeError> {
    // 当前字节索引初始化为0
    let mut curr = 0;
    let num_instructions = read_u16(&mut curr, data)?; // 读取指令数量
    let pc = curr; // 保存当前指针位置

    // 遍历事务指令
    for i in 0..num_instructions as usize {
        // 移动指针到正确位置
        curr = pc + i * 2;
        curr = read_u16(&mut curr, data)? as usize; //读取指令的偏移

        // 跳过账户
        let num_accounts = read_u16(&mut curr, data)? as usize;
        curr += num_accounts * 33; // 跳过账户数据

        // Read the instruction program id
        let program_id = read_pubkey(&mut curr, data)?;

        // Introspect on the first noop instruction
        if program_id.eq(&NOOP_PROGRAM_ID) {
            // Retrun address read from instruction data
            curr += 2;
            let address = read_pubkey(&mut curr, data)?;
            return Ok(Some(address));
        }
    }

    // Default return none
    Ok(None)
}
