use pinocchio::{
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    AccountView,
    ProgramResult,
};

use crate::constants::ID;
use crate::events::{emit_event, SubscriptionExpiredEvent};
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
    
    let subscription_address = *subscription_account.address();

    let (has_expired, expires_at_ts) = {
        let data = subscription_account.try_borrow()?;
        let view = SubscriptionPeriodView::load(&data)?;

        if !view.is_cancelled() {
            return Err(ProgramError::InvalidAccountData);
        }

        let current_ts = Clock::get()?.unix_timestamp;
        let expires_at_ts = view.expires_at_ts();

        (current_ts > expires_at_ts, expires_at_ts)
    };

    if has_expired {
        let event = SubscriptionExpiredEvent {
            subscription: subscription_address,
            expired_at_ts: expires_at_ts,
        };

        emit_event(&ID, event_authority, self_program, &event.to_bytes())?;
    }

    Ok(())
}