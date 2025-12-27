# Settlement Rails (Chain-Agnostic)

## Purpose
Provide a flexible settlement abstraction that supports:
- real-time payouts
- escrow-based trust
- cross-border value transfer
while keeping the **user experience fully fiat-native**.

Blockchain is used as an *invisible backend rail*, not as a user-facing feature.

---

## Design Principles
- **Chain-agnostic by default**: initial implementations may target a specific chain,
  but the interface does not assume exclusivity.
- **Non-custodial**: Kori never holds user funds.
- **Event-driven**: settlement is triggered by verified events, not manual ops.
- **Composable**: can support tips, deposits, bookings, or future settlement flows.

---

## Core Settlement Flow ("Digital Handshake")

1. **Commitment**
   - Visitor initiates a paid action (tip, booking deposit, experience fee).
   - Payment occurs in fiat (credit card / wallet).

2. **Escrow**
   - Funds are converted to a digital settlement unit (e.g. USDC).
   - Value is locked in a non-custodial escrow contract.

3. **Verification**
   - Both parties confirm completion via:
     - user action (e.g. "End Tour")
     - optional location/time verification
     - optional dispute window

4. **Settlement**
   - Escrow releases funds directly to the local guide.
   - Guide receives local fiat via off-ramp.

---

## Why Not Traditional Platforms
Legacy platforms rely on:
- consolidated batching
- multi-day settlement delays
- platform-controlled custody
- opaque FX spreads

Kori's settlement rail enables:
- real-time or same-day settlement
- transparent value flow
- reduced intermediary friction
- programmable trust guarantees

---

## Implementation Notes
- Initial versions may simulate escrow off-chain for UX testing.
- On-chain deployment is gated behind:
  - sufficient volume
  - regulatory clarity
  - partner readiness
- Settlement modules are intentionally isolated from chat and content logic.

---

## Status
Early-stage abstraction.
This module defines interfaces and flow, not final chain selection.
