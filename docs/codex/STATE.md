# Continuity

## Completed

- GitHub confirms local `catalyst-crank` is renamed `dakewamama/arm`.
- Root crate converted to protocol-neutral ARM; no nested crate or new repository.
- Schema 0.1, ten canonical fixtures, conservative lifecycle and attenuation evaluation.
- Old program remains available in Git history at `7957c8145c81a4d44ab9d1a94042bdb8a5f27c69`.

## Verified upstream

- Permit2 one-shot invalidation semantics: targeted source/tests, not executed locally.
- Initial Subscriptions source study began after an incomplete SUB-0 review; no
  adapter or upstream source edits were made. Final local SUB-0 review now unlocks study.

## Test status

- ARM: fmt and strict Clippy pass; 12 tests pass, including canonical JSON round trips,
  malformed/version rejection, lifecycle boundaries, recurring arithmetic/staleness,
  one-shot consumption and implication soundness. Git diff whitespace check passes.
- SDK baseline: 3 tests and TypeScript check pass.
- SDK: 42 Rust tests across ABI and native token adapters; 3 Bun tests and TypeScript
  check pass locally. These results do not establish hosted CI success.
- Cataloger resolver: 7 tests plus 23 legacy tests pass; fmt and strict Clippy pass.

## Real blockers

- Cataloger hosted CI is EXTERNALLY BLOCKED by an account billing lock. This does not
  invalidate exact local gate evidence; the owner explicitly authorized progression.
  Architecture book remains unlocated.
- GitHub confirms catalyst-indexer was renamed cataloger; there is no distinct indexer
  among these checkouts.

## Next critical path

- ARM, SDK ABI and Cataloger resolver milestones committed and pushed.
- Native SPL/Token-2022 fixtures and revoke round trips are pushed at SDK 323b873.
- Cataloger correction a88dbd8 removes ARM semantics and clarifies support boundaries.
- SDK Freeze/Thaw now use Direct operational authority; ARM remains unchanged.
- Final local gate: ARM 12, Cataloger 30 and SDK 42 Rust tests, 3 Bun tests, typecheck,
  formatting and strict Clippy pass. Next: Subscriptions maintainer study, no adapter.
- Schema fixed at 0.1; protocol fixtures must justify refinements.
- Reuse audit recorded in catalyst-sdk/docs/research/REUSE_BOUNDARIES.md.
