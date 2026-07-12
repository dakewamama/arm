use pinocchio::address::declare_id;

declare_id!("11111111111111111111111111111111");

// pub const SUBSCRIPTION_PROGRAM_ID: [u8; 32] = [
//     //derive it alonge the line
// ];

pub const PERIOD_HOURS_OFFSET: usize = 115;
pub const CURRENT_PERIOD_START_TS_OFFSET: usize = 139;
pub const EXPIRES_AT_TS_OFFSET: usize = 147;
pub const SUBSCRIPTION_DELEGATION_LEN: usize = 155;

pub const SECS_PER_HOUR: i64 = 3600;