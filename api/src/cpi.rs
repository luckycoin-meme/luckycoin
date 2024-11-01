use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, rent::Rent,
    sysvar::Sysvar,
};

/// 创建一个新的程序派生地址 (PDA)。
#[inline(always)]
pub fn create_pda<'a, 'info>(
    target_account: &'a AccountInfo<'info>, // 目标账户信息，指向要创建的 PDA
    owner: &Pubkey, // 指向该账户的所有者（程序的公钥）
    space: usize, // 账户所需的存储空间大小
    pda_seeds: &[&[u8]], // 用于生成 PDA 的种子数据
    system_program: &'a AccountInfo<'info>, // 系统程序的账户信息
    payer: &'a AccountInfo<'info>, // 付款账户，用于支付账户创建的费用
) -> ProgramResult {
    let rent = Rent::get()?; // 获取当前的租金信息

    if target_account.lamports().eq(&0) {
        // 如果目标账户的余额为零，则创建新账户
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                payer.key, // 付款者的公钥
                target_account.key, // 目标账户的公钥
                rent.minimum_balance(space), // 计算租金免除所需的最小余额
                space as u64, // 账户需要的空间
                owner, // 账户的所有者
            ),
            &[
                payer.clone(), // 付款者账户
                target_account.clone(), // 目标账户
                system_program.clone(), // 系统程序账户
            ],
            &[pda_seeds], // 使用的种子
        )?;
    } else {
        // 如果目标账户的余额非零：

        // 1) 转移足够的 lamports 以满足租金免除要求
        let rent_exempt_balance = rent
            .minimum_balance(space) // 计算当前空间的最低租金
            .saturating_sub(target_account.lamports()); // 计算需要转移的余额
        if rent_exempt_balance.gt(&0) {
            // 如果需要转移的余额大于零
            solana_program::program::invoke(
                &solana_program::system_instruction::transfer(
                    payer.key, // 付款者的公钥
                    target_account.key, // 目标账户的公钥
                    rent_exempt_balance, // 转移的余额
                ),
                &[
                    payer.clone(), // 付款者账户
                    target_account.clone(), // 目标账户
                    system_program.clone(), // 系统程序账户
                ],
            )?;
        }

        // 2) 为账户分配空间
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::allocate(target_account.key, space as u64),
            &[target_account.clone(), system_program.clone()], // 目标账户和系统程序账户
            &[pda_seeds], // 使用的种子
        )?;

        // 3) 将我们的程序设置为账户的所有者
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::assign(target_account.key, owner),
            &[target_account.clone(), system_program.clone()], // 目标账户和系统程序账户
            &[pda_seeds], // 使用的种子
        )?;
    }

    Ok(()) // 成功返回
}