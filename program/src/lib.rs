mod claim;
mod close;
mod initialize;
mod mine;
mod open;
mod reset;
mod stake;
mod update;
mod upgrade;
mod health;

use claim::*;
use close::*;
use initialize::*;
use mine::*;
use open::*;
use reset::*;
use stake::*;
use update::*;
use upgrade::*;

use luckycoin_api::instruction::*;
use solana_program::{self, account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use crate::health::process_health;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    if program_id.ne(&luckycoin_api::id()) {
        return Err(ProgramError::IncorrectProgramId); 
    }
    let (tag, data) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?; // 如果数据为空，则返回错误

    match LuckycoinInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        LuckycoinInstruction::Claim => process_claim(accounts, data)?,
        LuckycoinInstruction::Close => process_close(accounts, data)?,
        LuckycoinInstruction::Mine => process_mine(accounts, data)?,
        LuckycoinInstruction::Open => process_open(accounts, data)?,
        LuckycoinInstruction::Reset => process_reset(accounts, data)?,
        LuckycoinInstruction::Stake => process_stake(accounts, data)?,
        LuckycoinInstruction::Update => process_update(accounts, data)?,
        LuckycoinInstruction::Upgrade => process_upgrade(accounts, data)?,
        LuckycoinInstruction::Health => process_health(accounts, data)?,
        LuckycoinInstruction::Initialize => process_initialize(accounts, data)?,
    }
    Ok(())
}
