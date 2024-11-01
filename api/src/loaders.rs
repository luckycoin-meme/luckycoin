use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};
use solana_program::program_pack::Pack;
use spl_token::state::Mint;
// 引入 Solana 的相关模块
use steel::*;  // 引入 steel 库，可能用于处理指令和账户元数据

use crate::{  // 引入当前模块中的常量和状态定义
              consts::*,
              state::{Bus, Config, Proof, Treasury},
};

/// 检查账户是否为签名者
pub fn load_signer(info: &AccountInfo<'_>) -> Result<(), ProgramError> {
    if !info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

/// Errors if:
/// - Account is not writable.
pub fn load_any(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not the sysvar address.
/// - Account cannot load with the expected address.
pub fn load_sysvar(info: &AccountInfo<'_>, key: Pubkey) -> Result<(), ProgramError> {
    if info.owner.ne(&sysvar::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    load_account(info, key, false)
}

/// Errors if:
/// - Address does not match the expected value.
/// - Expected to be writable, but is not.
pub fn load_account(
    info: &AccountInfo<'_>,
    key: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.key.ne(&key) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Address does not match the expected mint address.
/// - Data is empty.
/// - Data cannot deserialize into a mint account.
/// - Expected to be writable, but is not.
pub fn load_mint(
    info: &AccountInfo<'_>,
    address: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.key.ne(&address) {
        return Err(ProgramError::InvalidSeeds);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    Mint::unpack(&info.data.borrow())?;

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}


/// Errors if:
/// - Address does not match PDA derived from provided seeds.
/// - Cannot load as an uninitialized account.
pub fn load_uninitialized_pda(
    info: &AccountInfo<'_>,
    seeds: &[&[u8]],
    bump: u8,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let pda = Pubkey::find_program_address(seeds, program_id);

    if info.key.ne(&pda.0) {
        return Err(ProgramError::InvalidSeeds);
    }

    if bump.ne(&pda.1) {
        return Err(ProgramError::InvalidSeeds);
    }

    load_system_account(info, true)
}

/// Errors if:
/// - Owner is not the system program.
/// - Data is not empty.
/// - Account is not writable.
pub fn load_system_account(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    if info.owner.ne(&system_program::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !info.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Data is empty.
/// - Data cannot deserialize into a token account.
/// - Token account owner does not match the expected owner address.
/// 加载并验证代币账户。
/// 错误条件：
pub fn load_token_account(
    info: &AccountInfo<'_>,  // 账户信息
    owner: Option<&Pubkey>,   // 可选的账户所有者地址
    mint: &Pubkey,            // 预期的铸币地址
    is_writable: bool,        // 是否期望账户可写
) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为 SPL Token 程序
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);  // 如果不是，返回无效账户所有者的错误
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);  // 如果为空，返回未初始化账户的错误
    }

    // 获取账户数据并尝试反序列化为 SPL Token 账户
    let account_data = info.data.borrow();
    let account = spl_token::state::Account::unpack(&account_data)?;

    // 检查代币账户的铸币地址是否匹配预期的铸币地址
    if account.mint.ne(&mint) {
        return Err(ProgramError::InvalidAccountData);  // 如果不匹配，返回无效账户数据的错误
    }

    // 如果提供了所有者地址，检查账户所有者是否匹配
    if let Some(owner) = owner {
        if account.owner.ne(owner) {
            return Err(ProgramError::InvalidAccountData);  // 如果不匹配，返回无效账户数据的错误
        }
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);  // 如果不可写，返回无效账户数据的错误
    }

    Ok(())  // 所有检查通过，返回成功
}

/// 加载并验证程序账户。
pub fn load_program(info: &AccountInfo<'_>, key: Pubkey) -> Result<(), ProgramError> {
    // 检查账户地址是否与预期的地址匹配
    if info.key.ne(&key) {
        return Err(ProgramError::IncorrectProgramId);  // 如果不匹配，返回错误：程序 ID 不正确
    }

    // 检查账户是否可执行
    if !info.executable {
        msg!("infoKey {:?}",info.key);
        msg!("key {:?}",key);
        msg!("info.executable {:?}",info.executable);
        msg!("load_program executable error");
        return Err(ProgramError::InvalidAccountData);  // 如果不可执行，返回无效账户数据的错误
    }

    Ok(())  // 所有检查通过，返回成功
}

/// 加载 bus 账户并进行验证。
/// 错误条件：
pub fn load_bus(info: &AccountInfo<'_>, id: u64, is_writable: bool) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户地址是否匹配预期的 bus 地址
    if info.key.ne(&BUS_ADDRESSES[id as usize]) {
        return Err(ProgramError::InvalidSeeds);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 尝试将账户数据反序列化为 Bus 结构体
    let bus_data = info.data.borrow();
    let bus = Bus::try_from_bytes(&bus_data)?;

    // 检查反序列化后的 Bus ID 是否匹配
    if bus.id.ne(&id) {
        return Err(ProgramError::InvalidAccountData);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载任意 bus 账户并进行验证。
/// 错误条件：
pub fn load_any_bus(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 检查数据是否可以反序列化为 Bus 账户
    if info.data.borrow()[0].ne(&(Bus::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    // 检查地址是否在有效的 bus 地址集合中
    if !BUS_ADDRESSES.contains(info.key) {
        return Err(ProgramError::InvalidSeeds);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载 config 账户并进行验证。
/// 错误条件：
pub fn load_config(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户地址是否匹配预期的 config 地址
    if info.key.ne(&CONFIG_ADDRESS) {
        return Err(ProgramError::InvalidSeeds);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 检查数据是否可以反序列化为 Config 账户
    if info.data.borrow()[0].ne(&(Config::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载 proof 账户并进行验证。
/// 错误条件：
pub fn load_proof(
    info: &AccountInfo<'_>,
    authority: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 尝试将账户数据反序列化为 Proof 结构体
    let proof_data = info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;

    // 检查证明的授权者是否匹配
    if proof.authority.ne(&authority) {
        return Err(ProgramError::InvalidAccountData);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载带矿工地址的 proof 账户并进行验证。
/// 错误条件：
pub fn load_proof_with_miner(
    info: &AccountInfo<'_>,
    miner: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 尝试将账户数据反序列化为 Proof 结构体
    let proof_data = info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;

    // 检查证明的矿工地址是否匹配
    if proof.miner.ne(&miner) {
        return Err(ProgramError::InvalidAccountData);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载任意 proof 账户并进行验证。
/// 错误条件：
pub fn load_any_proof(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 检查数据是否可以反序列化为 Proof 账户
    if info.data.borrow()[0].ne(&(Proof::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载 treasury 账户并进行验证。
/// 错误条件：
pub fn load_treasury(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    // 检查账户的所有者是否为当前程序
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // 检查账户地址是否匹配预期的 treasury 地址
    if info.key.ne(&TREASURY_ADDRESS) {
        return Err(ProgramError::InvalidSeeds);
    }

    // 检查账户数据是否为空
    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 检查数据是否可以反序列化为 Treasury 账户
    if info.data.borrow()[0].ne(&(Treasury::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    // 检查是否期望可写，但账户不可写
    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// 加载 treasury 代币账户并进行验证。
/// 错误条件：
pub fn load_treasury_tokens(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    // 检查账户地址是否匹配预期的 treasury 代币地址
    if info.key.ne(&TREASURY_TOKENS_ADDRESS) {
        return Err(ProgramError::InvalidSeeds);
    }

    // 调用 load_token_account 函数验证代币账户
    load_token_account(info, Some(&TREASURY_ADDRESS), &MINT_ADDRESS, is_writable)
}