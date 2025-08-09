# Contributing to OCOS Router Protocol (ORP)

First, thank you for your interest in contributing! 🤝  
By contributing to the OCOS Router Protocol (ORP) project, you are helping build **a secure, modular, and universal cross-chain routing standard** that benefits the entire blockchain ecosystem.

This document outlines a clear, logical, and experience-driven contribution process.

---

## 📋 Table of Contents
1. [Project Philosophy and Goals](#project-philosophy-and-goals)  
2. [Before You Start](#before-you-start)  
3. [Workflow](#workflow)  
4. [Code Standards](#code-standards)  
5. [Testing and Quality Assurance](#testing-and-quality-assurance)  
6. [Pull Request Guidelines](#pull-request-guidelines)  
7. [Git Commit Conventions](#git-commit-conventions)  
8. [Security and Vulnerability Reporting](#security-and-vulnerability-reporting)  
9. [Ethics and Community Conduct](#ethics-and-community-conduct)  

---

## Project Philosophy and Goals
- **Universality:** ORP should support all EVM chains, TRON, and non-EVM chains (Solana, Sui, Aptos, TON, NEAR).  
- **Modularity:** Bridge and DEX adapters should be easily added or removed.  
- **Security:** Prevent replay attacks, MEV risks, signature forgeries, and unauthorized adapter calls.  
- **Open-source Quality:** Code must be readable, well-documented, and audit-friendly.

---

## Before You Start
- **Read the Documentation:** Review the [README.md](README.md) and project structure.  
- **Set Up Your Development Environment:**
  - Solidity development: [Foundry](https://book.getfoundry.sh/)
  - TypeScript SDK: Node.js 18+
  - Solana program development: Anchor CLI (optional)
- **Discuss Changes:** For major features or changes, open a GitHub Discussion or Issue before starting.

---

## Workflow
1. **Fork** the repository into your GitHub account.  
2. **Create a branch** — use `feat/<topic>`, `fix/<topic>`, or `docs/<topic>` naming conventions.  
3. **Develop your feature/fix** — keep changes focused on one scope.  
4. **Run all tests** — ensure all tests pass before committing.  
5. **Open a Pull Request** — fill out the PR template.

---

## Code Standards
- **Solidity:**
  - Version: `^0.8.24`
  - Follow OpenZeppelin standards
  - Use **camelCase** for function and variable names
  - Add `NatSpec` comments for all public functions and data structures
- **TypeScript SDK:**
  - `strict` mode enabled
  - Target: ES2021+
  - Format with `prettier` and `eslint`
- **Rust (Solana):**
  - Use Anchor framework structure
  - Use `borsh` serialization
  - Document functions with `///` comments

---

## Testing and Quality Assurance
- **Unit Tests:** Required for every new feature or fix.  
- **Fuzz Tests:** For bridge and DEX adapters, validate with random inputs.  
- **Fork Tests:** Run against at least one real mainnet RPC (e.g., BSC/Ethereum).  
- **Coverage:** Target at least **90% test coverage** for merged PRs.

---

## Pull Request Guidelines
- **Be focused:** PRs should target a single main change.  
- **Explain the “why”:** Describe the problem solved and reasoning for your solution.  
- **Show test results:** Include a summary of your test run in the PR description.  
- **Use labels:** Apply `feature`, `bug`, `security`, or `docs` labels where appropriate.

---

## Git Commit Conventions
We follow the **Conventional Commits** format:

feat: add new DEX adapter
fix: correct cross-chain nonce validation
docs: update README.md
test: add slippage test for RouterCore

- Keep the subject line short (50–72 characters).
- Use a blank line before adding more detailed explanations.

---

## Security and Vulnerability Reporting
- **Do not** open public PRs for vulnerabilities.  
- Email: `security@ocos.io` (PGP encryption preferred).  
- We follow a private, coordinated disclosure process and credit researchers unless anonymity is requested.

---

## Ethics and Community Conduct
- Be respectful, inclusive, and collaborative.  
- Give constructive feedback in code reviews.  
- Avoid “my way is the only way” arguments — stay open to discussion.  
- Follow the [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

💡 **Golden Rule:** Write code so that you (or anyone else) can read and fully understand it six months from now.
