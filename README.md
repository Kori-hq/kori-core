# Kori Core

Kori is a "social settlement layer" for global travel: real locals help real travelers before & after they land, and every helpful conversation becomes a trusted travel asset.

This repository contains the core system modules described in the Kori deck:
- The AI Asset Generation Engine (unstructured chat → structured business page)
- Closed-loop micro-rewards (C-token ledger and referral click rewards)
- Settlement rail abstractions (optional escrow / verification / payout)
- Trust & safety primitives (anti-click-game, verification, abuse controls)

## Core Loop (v1)
1) Visitors and locals chat (group chat)
2) A user references a place using a hashtag (e.g. #kkanbujongno)
3) The system extracts the entity, calls Places API, asks user confirmation
4) A business page is created and linked back to the chat
5) When visitors click outbound links, the referrer earns C-tokens (closed-loop)

## Repo Layout
- docs/ : system explanation + diagrams
- modules/ : implementation units (API-agnostic where possible)
- prototypes/ : mock data + demo scripts

## Status
Early-stage scaffold. Modules are intentionally separated so we can implement in phases
while keeping the system auditable and extensible.
