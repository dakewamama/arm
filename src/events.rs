use const_crypto::ed25519;
use pinocchio::cpi::{invoke_signed, Seed, Signer};
use pinocchio::error::ProgramError;
use pinocchio::instruction::{InstructionAccount, InstructionView};
use pinocchio::{AccountView, Address, ProgramResult};

pub const EVENT_AUTHORITY_SEED: &[u8] = b"event_authority";

pub const EVENT_IX_TAG: u64 = 0x1d9acb512ea545e4;
pub const EVENT_IX_TAG_LE: [u8; 8] = EVENT_IX_TAG.to_le_bytes();

pub const EVENT_DISCRIMINATOR_LEN: usize = 9;
pub const EMIT_EVENT_IX_DISC: u8 = 228;


pub mod event_authority_pda {
    use super::*;

    const EVENT_AUTHORITY_AND_BUMP: ([u8; 32], u8) =
        ed25519::derive_program_address(&[EVENT_AUTHORITY_SEED], crate::constants::ID.as_array());

    pub const ID: Address = Address::new_from_array(EVENT_AUTHORITY_AND_BUMP.0);
    pub const BUMP: u8 = EVENT_AUTHORITY_AND_BUMP.1;
}

pub const PERIOD_ADVANCED_DISC: u8 = 0;
pub const SUBSCRIPTION_EXPIRED_DISC: u8 = 1;

pub const EVENT_DATA_LEN: usize = 40;
pub const EVENT_WIRE_LEN: usize = EVENT_DISCRIMINATOR_LEN + EVENT_DATA_LEN;

pub struct PeriodAdvancedEvent {
    pub subscription: Address,
    pub new_period_start_ts: i64,
}

impl PeriodAdvancedEvent {
    pub fn to_bytes(&self) -> [u8; EVENT_WIRE_LEN] {
        let mut buf = [0u8; EVENT_WIRE_LEN];
        buf[..8].copy_from_slice(&EVENT_IX_TAG_LE);
        buf[8] = PERIOD_ADVANCED_DISC;
        buf[9..41].copy_from_slice(self.subscription.as_array());
        buf[41..49].copy_from_slice(&self.new_period_start_ts.to_le_bytes());
        buf
    }
}

pub struct SubscriptionExpiredEvent {
    pub subscription: Address,
    pub expired_at_ts: i64,
}

impl SubscriptionExpiredEvent {
    pub fn to_bytes(&self) -> [u8; EVENT_WIRE_LEN] {
        let mut buf = [0u8; EVENT_WIRE_LEN];
        buf[..8].copy_from_slice(&EVENT_IX_TAG_LE);
        buf[8] = SUBSCRIPTION_EXPIRED_DISC;
        buf[9..41].copy_from_slice(self.subscription.as_array());
        buf[41..49].copy_from_slice(&self.expired_at_ts.to_le_bytes());
        buf
    }
}

#[inline(always)]
pub fn verify_event_authority(account: &AccountView) -> Result<(), ProgramError> {
    if account.address() != &event_authority_pda::ID {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

pub fn emit_event(
    program_id: &Address,
    event_authority: &AccountView,
    self_program: &AccountView,
    event_data: &[u8],
) -> ProgramResult {
    verify_event_authority(event_authority)?;

    let bump = [event_authority_pda::BUMP];
    let signer_seeds: [Seed; 2] = [Seed::from(EVENT_AUTHORITY_SEED), Seed::from(&bump)];
    let signer = Signer::from(&signer_seeds);

    let accounts = [InstructionAccount::readonly_signer(event_authority.address())];

    let instruction = InstructionView {
        program_id,
        data: event_data,
        accounts: &accounts,
    };

    invoke_signed::<2, _>(&instruction, &[event_authority, self_program], &[signer])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_first_byte_matches_emit_event_disc() {
        assert_eq!(EVENT_IX_TAG_LE[0], EMIT_EVENT_IX_DISC);
    }

    #[test]
    fn period_advanced_wire_format() {
        let event = PeriodAdvancedEvent {
            subscription: Address::new_from_array([7u8; 32]),
            new_period_start_ts: 1_700_000_000,
        };
        let bytes = event.to_bytes();

        assert_eq!(&bytes[..8], &EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], PERIOD_ADVANCED_DISC);
        assert_eq!(&bytes[9..41], &[7u8; 32]);
        assert_eq!(&bytes[41..49], &1_700_000_000i64.to_le_bytes());
    }

    #[test]
    fn expired_wire_format() {
        let event = SubscriptionExpiredEvent {
            subscription: Address::new_from_array([9u8; 32]),
            expired_at_ts: 1_700_100_000,
        };
        let bytes = event.to_bytes();

        assert_eq!(bytes[8], SUBSCRIPTION_EXPIRED_DISC);
        assert_eq!(&bytes[9..41], &[9u8; 32]);
        assert_eq!(&bytes[41..49], &1_700_100_000i64.to_le_bytes());
    }

    #[test]
    fn events_produce_different_wire_bytes() {
        let a = PeriodAdvancedEvent {
            subscription: Address::new_from_array([1u8; 32]),
            new_period_start_ts: 1,
        };
        let b = SubscriptionExpiredEvent {
            subscription: Address::new_from_array([1u8; 32]),
            expired_at_ts: 1,
        };
        assert_ne!(a.to_bytes(), b.to_bytes());
    }
}