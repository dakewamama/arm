# catalyst-crank

A Pinocchio `no_std` Solana program that reads [Solana Subscriptions Program](https://github.com/solana-foundation/subscriptions) account state and emits lifecycle events via Anchor-compatible self-CPI.

Program ID: `4QHqY9xtVyGmHVM9h5DD1i4zXQR7KabgahyQsY8eCV1o`

Devnet. Unaudited.

## Instructions

| Disc | Instruction | Condition | Emits |
|---|---|---|---|
| 0 | `advance_period` | `now > current_period_start_ts + period_length` | `PeriodAdvancedEvent` |
| 1 | `mark_expired` | `now > expires_at_ts` on a cancelled subscription | `SubscriptionExpiredEvent` |
| 228 | | self-CPI callback | no-op |

Both are permissionless and read-only. Neither writes to the Foundation's accounts.

Accounts, in order: the `SubscriptionDelegation` being read, the event authority PDA, the program itself.

## Design decisions

**Read by offset, not by mirrored struct.**

`SubscriptionDelegation` is read as raw bytes at named offsets rather than by declaring a matching `repr(C, packed)` struct.

```
Header       107 bytes   discriminator, version, bump, delegator, delegatee, payer, init_id
PlanTerms     24 bytes   amount, period_hours, created_at
own fields    24 bytes   amount_pulled_in_period, current_period_start_ts, expires_at_ts
                 155     matches SUBSCRIPTION_DELEGATION_LEN_V1
```

Only three fields are read:

```rust
PERIOD_HOURS_OFFSET            = 115
CURRENT_PERIOD_START_TS_OFFSET = 139
EXPIRES_AT_TS_OFFSET           = 147
```

A mirrored struct breaks silently if a field is inserted mid-layout. Every offset after it shifts, nothing fails to compile, and reads return plausible garbage. Three named offsets only break if those specific fields move. The Foundation exports offset constants for the `Header` portion themselves.

**Owner check before reading.**

The account is verified as owned by `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44` before any offset is read. Without it the program reads any 155-byte account and emits events about whatever it finds.

**Fixed-size event payloads.**

Events are a 32-byte address plus an 8-byte timestamp, known at compile time. `[u8; N]` instead of `alloc::vec::Vec` means no allocator and no heap.

**Same event tag as the Foundation.**

`EVENT_IX_TAG` is `0x1d9acb512ea545e4`, `Sha256("anchor:event")[..8]`, identical to theirs. Indexers filter by program ID first; the tag only marks an inner instruction as an event rather than a real call. A different tag would break existing decoders for no benefit.

**The 228 no-op.**

The tag's first little-endian byte is `0xe4`, which is 228. Emitting an event CPIs into the program itself, so the call arrives back at the entrypoint with discriminator 228. Without a no-op branch, every emission fails `InvalidInstructionData` and reverts the transaction.

```rust
match *discriminator {
    0 => advance_period::process(accounts),
    1 => mark_expired::process(accounts),
    EMIT_EVENT_IX_DISC => Ok(()),
    _ => Err(ProgramError::InvalidInstructionData),
}
```

**Borrow scoped before CPI.**

Account data borrows drop before `emit_event` runs. Holding a borrow across a CPI is rejected at runtime.

**`nostd_panic_handler`, not the default.**

`entrypoint!` expands `default_panic_handler!`, which assumes `std`. A `no_std` program needs `nostd_panic_handler!()` declared separately or the BPF build fails with a missing `#[panic_handler]`.

**`advance_period` rejects cancelled, `mark_expired` requires it.**

`expires_at_ts` is zero while active and set on cancellation. A cancelled subscription shouldn't have its period advanced; an active one cannot expire.

## Build

```bash
cargo build-sbf   # target/deploy/catalyst_crank.so, ~6.2K
cargo test
```

The SBF toolchain ships its own Cargo that cannot parse v4 lockfiles.

`#![cfg_attr(not(test), no_std)]` keeps `std` available in tests for `Vec` while the program stays `no_std`.

## Scope

Observes and emits. Does not CPI into `transfer_subscription`, does not move funds, has no reward mechanism.

Tests run against constructed byte buffers. Offsets are verified against the Foundation's compile-time length assertion, not against dumped accounts.