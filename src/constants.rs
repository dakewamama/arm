use pinocchio::address::declare_id;
use pinocchio::Address;

declare_id!("4QHqY9xtVyGmHVM9h5DD1i4zXQR7KabgahyQsY8eCV1o");

pub const SUBSCRIPTIONS_PROGRAM_ID: Address = 
    solana_address::address!("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44");

pub const PERIOD_HOURS_OFFSET: usize = 115;
pub const CURRENT_PERIOD_START_TS_OFFSET: usize = 139;
pub const EXPIRES_AT_TS_OFFSET: usize = 147;
pub const SUBSCRIPTION_DELEGATION_LEN: usize = 155;

pub const SECS_PER_HOUR: i64 = 3600;