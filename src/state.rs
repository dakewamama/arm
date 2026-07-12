use pinocchio::error::ProgramError;

use crate::constants::{
    CURRENT_PERIOD_START_TS_OFFSET, EXPIRES_AT_TS_OFFSET, PERIOD_HOURS_OFFSET,
    SECS_PER_HOUR, SUBSCRIPTION_DELEGATION_LEN,
};

pub struct SubscriptionPeriodView<'a> {
    data: &'a [u8],
}

impl<'a> SubscriptionPeriodView<'a> {
    pub fn load(data: &'a [u8]) -> Result<Self, ProgramError> {
        if data.len() != SUBSCRIPTION_DELEGATION_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self { data })
    }

    pub fn period_hours(&self) -> u64 {
        let bytes: [u8; 8] = self.data[PERIOD_HOURS_OFFSET..PERIOD_HOURS_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(bytes)
    }

    pub fn current_period_start_ts(&self) -> i64 {
        let bytes: [u8; 8] = self.data
            [CURRENT_PERIOD_START_TS_OFFSET..CURRENT_PERIOD_START_TS_OFFSET + 8]
            .try_into()
            .unwrap();
        i64::from_le_bytes(bytes)
    }

    pub fn expires_at_ts(&self) -> i64 {
        let bytes: [u8; 8] = self.data[EXPIRES_AT_TS_OFFSET..EXPIRES_AT_TS_OFFSET + 8]
            .try_into()
            .unwrap();
        i64::from_le_bytes(bytes)
    }

    pub fn period_length_secs(&self) -> i64 {
        self.period_hours() as i64 * SECS_PER_HOUR
    }

    pub fn is_cancelled(&self) -> bool {
        self.expires_at_ts() != 0
    }

    pub fn period_has_elapsed(&self, current_ts: i64) -> bool {
        current_ts > self.current_period_start_ts() + self.period_length_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_account(period_hours: u64, period_start_ts: i64, expires_at_ts: i64) -> Vec<u8> {
        let mut data = vec![0u8; SUBSCRIPTION_DELEGATION_LEN];
        data[PERIOD_HOURS_OFFSET..PERIOD_HOURS_OFFSET + 8]
            .copy_from_slice(&period_hours.to_le_bytes());
        data[CURRENT_PERIOD_START_TS_OFFSET..CURRENT_PERIOD_START_TS_OFFSET + 8]
            .copy_from_slice(&period_start_ts.to_le_bytes());
        data[EXPIRES_AT_TS_OFFSET..EXPIRES_AT_TS_OFFSET + 8]
            .copy_from_slice(&expires_at_ts.to_le_bytes());
        data
    }

    #[test]
    fn reads_period_hours_correctly() {
        let data = make_test_account(24, 1_700_000_000, 0);
        let view = SubscriptionPeriodView::load(&data).unwrap();
        assert_eq!(view.period_hours(), 24);
    }

    #[test]
    fn reads_period_start_ts_correctly() {
        let data = make_test_account(24, 1_700_000_000, 0);
        let view = SubscriptionPeriodView::load(&data).unwrap();
        assert_eq!(view.current_period_start_ts(), 1_700_000_000);
    }

    #[test]
    fn not_cancelled_when_expires_at_is_zero() {
        let data = make_test_account(24, 1_700_000_000, 0);
        let view = SubscriptionPeriodView::load(&data).unwrap();
        assert!(!view.is_cancelled());
    }

    #[test]
    fn cancelled_when_expires_at_is_nonzero() {
        let data = make_test_account(24, 1_700_000_000, 1_700_100_000);
        let view = SubscriptionPeriodView::load(&data).unwrap();
        assert!(view.is_cancelled());
    }

    #[test]
    fn period_has_not_elapsed_before_period_length() {
        let data = make_test_account(24, 1_700_000_000, 0);
        let view = SubscriptionPeriodView::load(&data).unwrap();
        // 24 hours = 86400 secs, checking exactly at start + 1 hour
        assert!(!view.period_has_elapsed(1_700_000_000 + 3600));
    }

    #[test]
    fn period_has_elapsed_after_period_length() {
        let data = make_test_account(24, 1_700_000_000, 0);
        let view = SubscriptionPeriodView::load(&data).unwrap();
        // 24 hours = 86400 secs, checking after that has passed
        assert!(view.period_has_elapsed(1_700_000_000 + 86_401));
    }

    #[test]
    fn rejects_wrong_length_data() {
        let data = vec![0u8; 10];
        assert!(SubscriptionPeriodView::load(&data).is_err());
    }
}