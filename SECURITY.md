# Security Policy

Kori takes security and trust seriously, especially because the system
models user-to-user interactions, rewards, and settlement logic.

This repository is early-stage and not production-ready, but security
considerations are built into the architecture from the beginning.

---

## Scope

This policy applies to:
- core system modules
- prototypes and demo logic
- architectural documentation
- any future smart-contract or settlement-related code

---

## Reporting a Vulnerability

If you discover a security issue or potential vulnerability, please **do not**
open a public GitHub issue.

Instead, report it privately by emailing:

**jieun.noh@craters.co**  
*(If this address is not yet active, you may temporarily use the repository
maintainer’s contact information.)*

Please include:
- a clear description of the issue
- steps to reproduce (if applicable)
- potential impact
- any suggested mitigations

---

## Responsible Disclosure

We ask that you follow responsible disclosure practices:
- allow time for investigation and remediation
- avoid public disclosure until a fix or mitigation is available

We will acknowledge reports promptly and aim to communicate next steps clearly.

---

## Current Security Posture

At this stage:
- no production secrets are stored in this repository
- no real funds or private keys are handled
- settlement logic is simulated or abstracted
- reward systems are non-cash and closed-loop

As the project matures, this policy will evolve to cover:
- smart contract audits
- infrastructure hardening
- bounty programs (if applicable)

---

## Security Design Principles

Kori’s security approach emphasizes:
- separation of concerns
- minimal trust assumptions
- explicit state transitions
- human confirmation as a trust anchor
- auditable event and receipt flows

---

## Acknowledgements

We appreciate the security research community and welcome thoughtful,
good-faith contributions that help improve system safety and reliability.
