# OCOS Router Protocol – Pull Request Template

> Thank you for contributing to ORP! Please fill in all relevant sections.
> Incomplete PRs are okay, but mark missing parts as TODO so reviewers can help.

---

## Title

**Conventional Commit** style preferred (e.g., `feat(router): add rate limiter`, `fix(sdk): correct msgId derivation`)

---

## Summary

**What does this PR do?**  
A clear, concise overview of the change, the motivation, and expected impact.

- Problem:
- Proposed solution:
- Alternatives considered:
- Scope (components touched): `contracts/router`, `contracts/adapters`, `packages/sdk`, `solana/program`, `script`, `docs`, `test`

---

## Type of change

- [ ] feat (new feature)
- [ ] fix (bug fix)
- [ ] perf (performance / gas)
- [ ] refactor (no behavior change)
- [ ] docs (documentation only)
- [ ] test (tests only)
- [ ] chore (build, tooling, CI)
- [ ] security (vuln mitigation / hardening)
- [ ] breaking change (backward-incompatible)

---

## Linked issues / discussions

Closes #  
Relates to #  
Design doc / discussion:

---

## Technical details

**High-level design notes**  
- Entry points and control flow:
- Data structures / storage layout changes:
- Message format changes (if any):
- Cross-chain semantics affected (source/destination behavior):
- Error handling & edge cases:

**EVM (Solidity)**  
- Storage layout diff (if any):  
- Events added/changed:  
- Access control (roles/guards/timelocks):  
- Reentrancy / checks-effects-interactions considerations:  
- Upgradeability notes (proxy/UUPS/immutable):

**Solana (if applicable)**  
- Accounts touched / CPI calls:  
- Borsh schema changes:  
- Program IDs and seeds:

**SDK (TypeScript)**  
- API changes (breaking?):  
- Types & serialization changes:  
- Backwards compatibility notes:

---

## Security & risk assessment

- Threat model affected: replay, reentrancy, signature forgery, message spoofing, adapter misbehavior, MEV, rate-limits
- New trust assumptions introduced:
- External calls and untrusted inputs audited:
- Invariants kept (e.g., vault solvency, one-time message processing):
- Deny/allow lists updated:
- **Migration & rollback plan** (if deploy needed):
- **Operational runbook updates** (monitoring, alerts):

> If this PR touches security-sensitive paths, consider a private review first. See `SECURITY.md`.

---

## Gas / performance

- [ ] Gas report attached (Foundry `--gas-report`)  
- Summary of changes:
  - Hot path deltas:
  - Loop bounds / external calls:
  - Storage read/write counts:

Paste key excerpts or link to artifact.

---

## Tests

**Coverage**  
- [ ] Unit tests added/updated  
- [ ] Fork/integration tests (if applicable)  
- [ ] Cross-chain simulation (mock adapters)  
- [ ] Negative tests (reverts, bounds)

**How to run locally**
```bash
forge build
forge test -vv
# optional: with gas report
FOUNDRY_PROFILE=ci forge test --gas-report
```

**Artifacts / logs**  
- CI link:
- Failing tests (if any) with rationale/TODO:

---

## Deployment / releases

- Target chains & environments (e.g., BSC testnet, Ethereum Sepolia):
- Contracts to deploy / upgrade:
- Proxy admin / timelock steps:
- Post-deploy verification (events, receipts, canary tx):
- Backout plan:

---

## Docs & changelog

- [ ] README / docs updated
- [ ] CHANGELOG entry added
- User-facing changes summarized:

---

## Screenshots / diagrams (optional)

Attach images, sequence diagrams, or storage layout diffs that help reviewers.

---

## Checklist

- [ ] I read and follow `CONTRIBUTING.md`
- [ ] No secrets or private keys are committed
- [ ] Lint/build/tests pass locally
- [ ] Contract ABIs updated if interfaces changed
- [ ] Storage layout changes reviewed for upgrades
- [ ] Event and error names follow project style
- [ ] Proper error bubbling (custom errors preferred)
- [ ] Comments reflect current behavior
- [ ] All TODOs either completed or clearly tracked

---

## Reviewer notes

- Suggested reviewers:
- Areas of special attention (tricky logic, subtle invariants):
- Anything intentionally left for follow-up PRs:

---

## DCO / Sign-off (optional)

If your org requires DCO:

```
Signed-off-by: Your Name <you@example.com>
```
u
