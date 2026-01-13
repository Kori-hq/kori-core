# Trust & Safety Model

## Overview
Kori is a user-to-user system operating across borders.
Trust is enforced through **architecture**, not moderation at scale.

The goal is to:
- discourage abuse by design
- make honest behavior the easiest path
- preserve user dignity and platform neutrality

---

## Trust Layers

### 1. Identity Context (Soft)
- Account age
- Participation history
- Contribution quality
- Peer signals (likes, responses, confirmations)

No real-name or KYC requirement at the base layer.

---

### 2.1 Action Verification (Medium)
Used for value-triggering events:
- mutual confirmation (both sides)
- time-based completion
- optional location consistency

These checks are *contextual*, not permanent surveillance.

#### 2.2 Safe Return Confirmation (Privacy-Preserving)

**Goal:** Allow participants to confirm they returned safely (e.g., Tourist to hotel, Local to home) **without** the protocol learning the underlying address or storing continuous location history.

**High-level behavior**
- After an offline meetup, each participant may optionally publish a single safety signal:
  - `User_Status = SAFE` (or `UNSAFE` / `NO_CHECKIN` after a timeout)
- No public location history is created. The signal is contextual to the interaction window.

**Client-side setup (no server-side coordinates)**
- The user can set a **Safe Zone** (e.g., hotel/home) inside the app.
- Safe Zone coordinates are stored **only on-device** (encrypted storage) and are never sent to the server.

**OS-level monitoring**
- Android: Geofencing (e.g., `GeofencingClient`)
- iOS: Region Monitoring (Core Location)
- The OS is instructed: “Notify the app if the device enters this radius.”

**Trigger + signal**
- When the OS detects entry into the Safe Zone, the app can send a single message:
  - `User_Status: SAFE`
- Only the status is sent—**no coordinates**, no address, no route trace.

**Trust impact (soft signal)**
- Repeated successful safe-return confirmations can increase trust weight for future high-risk interactions.
- Missing or negative confirmations may temporarily reduce access to higher-risk interactions (without punitive bans).

**Privacy principles**
- Opt-in only
- No continuous tracking
- No storage of home/hotel addresses on servers
- No public “behavior scoring”; safety signals remain contextual and time-bounded

---

### 3. Economic Friction (Hard)
Abuse is discouraged through:
- rate limits
- diminishing rewards
- one-action-one-reward constraints
- delayed unlocks for new accounts

This avoids punitive bans where possible.

---

## Anti-Abuse Principles

### Closed-Loop Rewards
- C-tokens are non-cashout
- No direct arbitrage
- Value accrues through continued participation

This prevents farming and bot economies.

---

### Click Integrity
- Referral rewards require unique, valid sessions
- Repeat or automated clicks are ignored
- Suspicious patterns are logged, not immediately punished

---

### Progressive Trust
- New users have limited influence
- Trust increases through consistent, verified behavior
- High-trust users gain visibility, not raw power

---

## Disputes & Resolution
- Most interactions are designed to be low-stakes
- For paid experiences:
  - escrow delays allow dispute windows
  - resolution rules are predefined and auditable
- Kori avoids subjective moderation where possible

---

## Data & Privacy
- No selling of personal data
- Minimal storage of location signals
- Trust signals are contextual, not global reputation scores

---

## Status
Trust & safety is iterative.
This document defines principles and primitives, not final enforcement thresholds.
