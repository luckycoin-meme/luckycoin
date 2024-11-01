use ore_api::{loaders::*, state::Proof};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};
use steel::*;

/// 关闭证明账户并将租金返回给所有者
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // 从提供的账户数组中加载必要的账户
    let [signer, proof_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys); // 如果账户数量不足，则返回错误
    };
    // 加载签名者账户并进行验证
    load_signer(signer)?;
    // 加载证明账户，确保其与签名者的公钥匹配
    load_proof(proof_info, signer.key, true)?;
    // 加载系统程序
    load_program(system_program, system_program::id())?;

    // 验证余额是否为零
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;
    if proof.balance.gt(&0) {
        return Err(ProgramError::InvalidAccountData); //如果余额大于零，则返回错误
    }
    drop(proof_data); // 释放对 proof_data 的借用。

    // 将证明账户的数据将重新分配为零
    proof_info.realloc(0, true)?;

    // 将剩余的 lamports 发送给签名者。
    **signer.lamports.borrow_mut() += proof_info.lamports();
    **proof_info.lamports.borrow_mut() = 0;

    Ok(())
}
