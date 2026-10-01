# Continuity

## Completed

- GitHub confirms local `catalyst-crank` is renamed `dakewamama/arm`.
- Root crate converted to protocol-neutral ARM; no nested crate or new repository.
- Schema 0.1, ten canonical fixtures, conservative lifecycle and attenuation evaluation.
- Old program remains available in Git history at `7957c8145c81a4d44ab9d1a94042bdb8a5f27c69`.

## Verified upstream

- Permit2 one-shot invalidation semantics: targeted source/tests, not executed locally.
- No Subscriptions source study; SUB-0 remains locked.

## Test status

- ARM: fmt and strict Clippy pass; 12 tests pass, including canonical JSON round trips,
  malformed/version rejection, lifecycle boundaries, recurring arithmetic/staleness,
  one-shot consumption and implication soundness. Git diff whitespace check passes.
- SDK baseline: 3 tests and TypeScript check pass.
- SDK semantic ABI: 6 Rust tests, 3 Bun tests and TypeScript check pass.
- Cataloger resolver: 7 tests plus 23 legacy tests pass; fmt and strict Clippy pass.

## Real blockers

- No blocker for the current reuse audit. Architecture book remains unlocated.
- GitHub confirms catalyst-indexer was renamed cataloger; there is no distinct indexer
  among these checkouts.

## Next critical path

- ARM, SDK ABI and Cataloger resolver milestones committed and pushed.
- Next: official token interface compatibility and native fixtures/revoke round trip.
- Schema fixed at 0.1; protocol fixtures must justify refinements.
- Reuse audit recorded in catalyst-sdk/docs/research/REUSE_BOUNDARIES.md.
