use luckycoin_api::{consts::*, error::LuckycoinError, instruction::Stake};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack,
};
use spl_token::state::Mint;
use steel::*;
use luckycoin_api::loaders::{load_mint, load_program, load_signer, load_token_account};

pub fn process_upgrade(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let args = Stake::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let [
        signer, 
        beneficiary_info, 
        mint_info, 
        mint_v1_info, 
        sender_info, 
        treasury_info, 
        token_program
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    // 加载受益人账户，确保其有效并与版本2的铸币地址相关联。
    load_token_account(
        beneficiary_info, 
        Some(&signer.key), 
        &MINT_ADDRESS, 
        true
    )?;
    // 加载版本2和版本1的铸币账户。
    load_mint(mint_info, MINT_ADDRESS, true)?;
    load_mint(mint_v1_info, MINT_V1_ADDRESS, true)?;
    // 加载发送者的代币账户，确保其有效并与版本1的铸币地址相关联
    load_token_account(
        sender_info, 
        Some(signer.key), 
        &MINT_V1_ADDRESS, 
        true
    )?;
    load_program(token_program, spl_token::id())?;

    // 燃烧版本1代币
    solana_program::program::invoke(
        &spl_token::instruction::burn(
            &spl_token::id(),
            sender_info.key,
            mint_v1_info.key,
            signer.key,
            &[signer.key],
            amount,
        )?,
        &[
            token_program.clone(),
            sender_info.clone(),
            mint_v1_info.clone(),
            signer.clone(),
        ],
    )?;

    // 计算铸造数量：
    // v1 token has 9 decimals. v2 token has 11.
    let amount_to_mint = amount.saturating_mul(100);

    // 检查最大供应量(检查当前版本2代币的供应量，确保铸造后的总供应量不超过最大供应量)
    let mint_data = mint_info.data.borrow();
    let mint = Mint::unpack(&mint_data)?;
    if mint.supply.saturating_add(amount_to_mint).gt(&MAX_SUPPLY) {
        return Err(LuckycoinError::MaxSupply.into());
    }

    // 铸造版本2代币
    drop(mint_data);
    mint_to_signed(
        mint_info,
        beneficiary_info,
        treasury_info,
        token_program,
        amount_to_mint,
        &[&[TREASURY, &[TREASURY_BUMP]]],
    )?;

    Ok(())
}
