use pinocchio::{
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    AccountView,
    ProgramResult,
};

use crate::state::SubscriptionPeriodView;

pub fn process(accounts: &mut [AccountView]) -> ProgramResult {
    let subscription_account = accounts
        .first()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let data = subscription_account.try_borrow()?;
    let view = SubscriptionPeriodView::load(&data)?;

    if view.is_cancelled() {
        return Err(ProgramError::InvalidAccountData);
    }

    let current_ts = Clock::get()?.unix_timestamp;

    if view.period_has_elapsed(current_ts) {
        // Event emission happens here in the next commit
    }

    Ok(())
}
