use ore_api::{consts::*, error::OreError, instruction::*, loaders::*, state::Proof};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};
use steel::*;

/// Claim将可认领的ORE从国库分配给矿工
pub fn process_claim(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // 解析传入的数据，将其转换为Claim结构体
    let args = Claim::try_from_bytes(data)?;
    // 从Claim结构体中提取金额，并将其转换为u64
    let amount = u64::from_le_bytes(args.amount);

    // 从提供的账户数组中加载必要的账户
    let [signer, beneficiary_info, proof_info, treasury_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys); // 如果账户数量不足，则返回错误
    };
    // 加载签名者账户并进行验证
    load_signer(signer)?;
    // 加载受益人的代币账户，确保其与预期的铸币地址匹配。
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    // 加载与签名者关联的证明账户
    load_proof(proof_info, signer.key, true)?;
    // 加载国库账户
    load_treasury(treasury_info, false)?;
    // 加载国库代币账户
    load_treasury_tokens(treasury_tokens_info, true)?;
    // 加载SPL代币程序
    load_program(token_program, spl_token::id())?;

    // 可变借用证明账户的数据，以便更新余额。
    let mut proof_data = proof_info.data.borrow_mut();
    // 将证明数据反序列化Proof结构体
    let proof = Proof::try_from_bytes_mut(&mut proof_data)?;
    // 更新矿工的余额，通过减去认领的金额
    proof.balance = proof
        .balance
        .checked_sub(amount)
        .ok_or(OreError::ClaimTooLarge)?; // 确保余额不会变为负数，如果金额过大则返回错误。

    // 从国库向受益人账户转移代币。
    transfer_signed(
        treasury_info,
        treasury_tokens_info,
        beneficiary_info,
        token_program,
        amount,
        &[&[TREASURY, &[TREASURY_BUMP]]], // 用于转账的签名者种子。
    )?;

    Ok(())// 返回成功。
}