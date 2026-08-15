#![cfg_attr(not(test), no_std)]

mod constants;
mod state;
mod instructions;
mod events;

use pinocchio::{
    error::ProgramError,
    AccountView,
    ProgramResult,
};
use events::EMIT_EVENT_IX_DISC;

pinocchio::entrypoint!(process_instruction);
pinocchio::nostd_panic_handler!();

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
        1 => instructions::mark_expired::process(accounts),
        EMIT_EVENT_IX_DISC => Ok(()),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}