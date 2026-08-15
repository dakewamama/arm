use pinocchio::{
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    AccountView,
    ProgramResult,
};

use crate::constants::{ID, SUBSCRIPTIONS_PROGRAM_ID};
use crate::events::{emit_event, PeriodAdvancedEvent};
use crate::state::SubscriptionPeriodView;

pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let subscription_account = accounts
        .first()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let event_authority = accounts
        .get(1)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let self_program = accounts
        .get(2)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    if subscription_account.owner() != &SUBSCRIPTIONS_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let subscription_address = *subscription_account.address();

    let (period_elapsed, new_period_start_ts) = {
        let data = subscription_account.try_borrow()?;
        let view = SubscriptionPeriodView::load(&data)?;

        if view.is_cancelled() {
            return Err(ProgramError::InvalidAccountData);
        }

        let current_ts = Clock::get()?.unix_timestamp;

        (
            view.period_has_elapsed(current_ts),
            view.current_period_start_ts() + view.period_length_secs(),
        )
    };

    if period_elapsed {
        let event = PeriodAdvancedEvent {
            subscription: subscription_address,
            new_period_start_ts,
        };

        emit_event(&ID, event_authority, self_program, &event.to_bytes())?;
    }

    Ok(())
}