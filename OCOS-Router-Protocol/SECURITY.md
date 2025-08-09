# Security Policy

## Overview
The **OCOS Router Protocol (ORP)** is designed for secure, auditable cross-chain routing.  
While we follow best practices in blockchain security, this repository contains **reference implementations** and **experimental SDKs** that may not yet be production-ready.  
Any deployment on mainnet should undergo **independent, professional audits** and rigorous testing.

Security is a **shared responsibility** between:
- Protocol developers (ensuring code correctness and audit readiness),
- Operators (deploying with proper key and infrastructure security),
- Integrators (DEX/bridge adapters),
- End-users (careful signing and verification of transactions).

---

## Scope of This Policy
This document covers:
1. **Vulnerability Disclosure Process**
2. **Secure Development Guidelines**
3. **Deployment and Operational Security**
4. **Incident Response Procedures**

---

## 1. Vulnerability Disclosure Process

We operate on a **private, time-boxed disclosure model**:

- **Do not** disclose vulnerabilities via public GitHub issues, PRs, or social media.
- Email all security reports to:  
  📧 **security@ocos.io**  
  PGP Key (optional): Available upon request.
- Include:
  - Description of the vulnerability
  - Steps to reproduce
  - Impact assessment (financial/technical)
  - Potential fixes or mitigations
  - Affected components (core, adapter, SDK, Solana program, etc.)

**Response Targets:**
- **Acknowledgement:** within 48 hours
- **Initial assessment:** within 5 business days
- **Patch or mitigation plan:** within 10 business days for high/critical issues
- **Public disclosure:** coordinated with the reporter after patch deployment and sufficient notice period to users

We may offer **bug bounty rewards** for eligible reports that are:
- Original
- Reproducible
- Within scope

---

## 2. Secure Development Guidelines

### Code Quality
- **Solidity:** Use the latest stable compiler; enable `optimizer` and `pragma` locking.
- Avoid deprecated patterns; follow **Checks-Effects-Interactions**.
- Minimize `approve` allowances; use **EIP-2612 Permit** or Permit2 where possible.
- Always use **ReentrancyGuard** or equivalent in state-modifying external functions.
- Validate `deadline` and `nonce` for replay protection.
- Limit `callData` execution to whitelisted targets in adapters.

### Dependencies
- Only import audited, well-maintained libraries (e.g., OpenZeppelin).
- Pin versions in `package.json` and `foundry.toml`.
- Regularly run `npm audit` / `forge update`.

### Testing
- Unit tests for all critical functions.
- Fork tests for on-chain adapter integrations.
- Fuzz testing for arbitrary input handling.
- Formal verification where feasible.

---

## 3. Deployment and Operational Security

### Key Management
- Use hardware wallets or multi-signature contracts for:
  - `guardian`
  - `feeManager`
  - Adapter registration
- Never deploy or manage contracts with unprotected private keys on hot machines.

### Contract Upgrades
- Use upgradeable proxies **only** if necessary and with:
  - Timelocked upgrade functions
  - Multi-sig confirmations
  - Public announcement of upcoming changes

### Adapter Control
- Maintain an allowlist of trusted adapters per deployment.
- Regularly audit bridge and DEX adapters for API changes or security advisories.

### Rate Limiting
- Enable per-token and per-transaction limits to mitigate flash-drain scenarios.
- For production, configure conservative daily volume caps.

---

## 4. Incident Response

If a critical vulnerability or exploit is detected:

1. **Activate Pause:**
   - Guardian calls `pause(true)` on RouterCore and affected adapters.

2. **Communicate:**
   - Notify integrators and partners via secure channels.
   - Publish a security advisory on the GitHub repo and official channels.

3. **Investigate:**
   - Collect on-chain and off-chain logs.
   - Identify attack vectors and impacted funds/users.

4. **Patch & Deploy:**
   - Issue hotfix contracts.
   - Re-audit before unpausing.

5. **Postmortem:**
   - Publish a transparent report with root cause, mitigation steps, and improvements.

---

## Out of Scope
- Social engineering attacks
- Physical security compromises
- Issues in external dependencies (bridges, DEXs) beyond our control — these should be reported to their maintainers.

---

## Final Notes
- Always **test on a testnet** before deploying to mainnet.
- Assume all blockchain interactions are public and immutable.
- The **cost of an insecure deployment** is irreversible financial loss.

> Security is not a one-time action — it is a continuous process.  
> Every participant in the ecosystem has a role in keeping the protocol safe.
