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
- Local catalyst-indexer (remote cataloger): 23 tests pass; pre-existing fmt/Clippy failures.

## Real blockers

- Separate indexer location and architecture book not yet identified.

## Next critical path

- Catalyst semantic ABI after ARM commit/push.
- Schema fixed at 0.1; protocol fixtures must justify refinements.
- Verify GitHub identities for remaining repositories before changing their responsibilities.
