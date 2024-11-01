mod claim;
mod close;
mod initialize;
mod mine;
mod open;
mod reset;
mod stake;
mod update;
mod upgrade;

use claim::*;
use close::*;
use initialize::*;
use mine::*;
use open::*;
use reset::*;
use stake::*;
use update::*;
use upgrade::*;

use ore_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

// 定义程序的入口点
solana_program::entrypoint!(process_instruction);

// 程序的主处理函数
pub fn process_instruction(
    program_id: &Pubkey, // 程序的公钥
    accounts: &[AccountInfo], // 账户的信息切片
    data: &[u8], // 传入的指令数据
) -> ProgramResult {
    // 检查传入的 program_id 是否与预期的程序 ID 匹配
    if program_id.ne(&ore_api::id()) {
        return Err(ProgramError::IncorrectProgramId); // 如果不匹配，返回错误
    }

    // 从数据中提取第一个字节（指令的标签）和剩余数据
    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?; // 如果数据为空，则返回错误

    // 根据指令标签选择相应的处理函数
    match OreInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        OreInstruction::Claim => process_claim(accounts, data)?,
        OreInstruction::Close => process_close(accounts, data)?,
        OreInstruction::Mine => process_mine(accounts, data)?,
        OreInstruction::Open => process_open(accounts, data)?,
        OreInstruction::Reset => process_reset(accounts, data)?,
        OreInstruction::Stake => process_stake(accounts, data)?,
        OreInstruction::Update => process_update(accounts, data)?,
        OreInstruction::Upgrade => process_upgrade(accounts, data)?,
        OreInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
