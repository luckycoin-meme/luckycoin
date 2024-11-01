use std::mem::size_of;

use luckycoin_api::{
    consts::*,
    instruction::Open,
    state::Proof,
    loaders::*,
};
use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    entrypoint::ProgramResult,
    keccak::hashv,
    program_error::ProgramError,
    slot_hashes::SlotHash,
    system_program,
    sysvar::{self, Sysvar},
};
use steel::*;
use luckycoin_api::cpi::create_pda;

pub fn process_open(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Open::try_from_bytes(data)?;

    // Load accounts.
    let [
        signer, 
        miner_info, 
        payer_info, 
        proof_info, 
        system_program, 
        slot_hashes_info
    ] = accounts 
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    load_any(miner_info, false)?;
    load_signer(payer_info)?;
    load_uninitialized_pda(
        proof_info,
        &[PROOF, signer.key.as_ref()],
        args.bump,
        &luckycoin_api::id(),
    )?;
    load_program(system_program, system_program::id())?;
    load_sysvar(slot_hashes_info, sysvar::slot_hashes::id())?;

    // Initialize proof.
    create_pda(
        proof_info,
        &luckycoin_api::id(),
        8 + size_of::<Proof>(),
        &[PROOF, signer.key.as_ref(), &[args.bump]],
        system_program,
        payer_info,
    )?;

    let clock = Clock::get().or(Err(ProgramError::InvalidAccountData))?;
    // 借用和初始化证明数据
    let mut proof_data = proof_info.data.borrow_mut();
    proof_data[0] = Proof::discriminator() as u8;
    
    let proof = Proof::try_from_bytes_mut(&mut proof_data)?;
    // 将矿工的签名者公钥设置为该证明的权威账户（authority），标识该矿工的所有权。
    proof.authority = *signer.key;
    proof.balance = 0;
    // 计算并设置挑战值(计算一个挑战值，它是通过哈希签名者的公钥和当前插槽哈希信息得到的)。这将用于后续验证挖矿
    proof.challenge = hashv(&[
        signer.key.as_ref(),
        &slot_hashes_info.data.borrow()[0..size_of::<SlotHash>()],
    ]).0;
    // 初始化最后哈希值(将最后的哈希值初始化为全零，表示尚未提交任何哈希)
    proof.last_hash = [0; 32];
    // 记录时间戳(将上次哈希提交和上次抵押的时间戳都设置为当前时间)
    proof.last_hash_at = clock.unix_timestamp;
    proof.last_stake_at = clock.unix_timestamp;
    // 设置矿工公钥
    proof.miner = *miner_info.key;
    // 初始化总哈希和总奖励
    proof.total_hashes = 0;
    proof.total_rewards = 0;

    Ok(())
}
