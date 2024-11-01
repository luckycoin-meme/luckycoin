use luckycoin_api::{loaders::*, state::Proof};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};
use steel::*;

pub fn process_update(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer, miner_info, proof_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    // 加载矿工信息账户，允许其为任何账户
    load_any(miner_info, false)?;
    // 加载证明账户，并检查其与签名者的关联性，确保签名者有权更新该证明账户
    load_proof(proof_info, signer.key, true)?;

    // 更新证明账户的矿工权限
    let mut proof_data = proof_info.data.borrow_mut();
    let proof = Proof::try_from_bytes_mut(&mut proof_data)?;
    proof.miner = *miner_info.key;

    Ok(())
}
