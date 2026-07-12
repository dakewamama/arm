#![cfg_attr(not(test), no_std)]

mod constants;
mod state;
mod instructions;

use pinocchio::{
    error::ProgramError,
    AccountView,
    ProgramResult,
};

pinocchio::entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &pinocchio::Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, _rest) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match *discriminator {
        0 => instructions::advance_period::process(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}