use luckycoin_api::{consts::*, instruction::Stake, loaders::*, state::Proof};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, sysvar::Sysvar,
};
use steel::*;

pub fn process_stake(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = Stake::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    let [
        signer, 
        proof_info, 
        sender_info, 
        treasury_tokens_info, 
        token_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    // 验证和加载账户
    load_signer(signer)?;
    // 加载并验证证明账户，确保与签名者相关联
    load_proof(proof_info, signer.key, true)?;
    // 加载发送者的代币账户，确保其有效性并与预定义的铸币地址相关联
    load_token_account(
        sender_info, 
        Some(signer.key), 
        &MINT_ADDRESS, 
        true
    )?;
    // 加载国币代币账户
    load_treasury_tokens(treasury_tokens_info, true)?;
    // 加载代币程序的ID
    load_program(token_program, spl_token::id())?;

    // 更新证明账户余额
    let mut proof_data = proof_info.data.borrow_mut();
    let proof = Proof::try_from_bytes_mut(&mut proof_data)?;
    proof.balance = proof.balance.checked_add(amount).unwrap();

    // 跟新抵押时间戳(获取当前时间戳，并将其设置为最后抵押的时间戳)
    let clock = Clock::get().or(Err(ProgramError::InvalidAccountData))?;
    proof.last_stake_at = clock.unix_timestamp;

    // 转移代币(将制定数量的代币从签名者账户转移到国库代币账户)
    transfer(
        signer,
        sender_info,
        treasury_tokens_info,
        token_program,
        amount,
    )?;

    Ok(())
}
