# Abstract Payments Ledger Plan

Date: 2026-05-04

## Goal

Represent marketplace payments through internal payment intents, transactions, and processor events without binding the codebase to a specific payment provider.

## Database

- [ ] Add payment provider enum/config abstraction.
- [ ] Add payment intent table with UUIDv7 primary key, user/provider references, amount, currency, lifecycle state, provider reference fields, metadata, and timestamps.
- [ ] Add transaction ledger table with immutable payment movements and state.
- [ ] Add processor event table for idempotent webhook/event ingestion with payload storage and processing status.
- [ ] Add indexes for user/provider/admin lookup and event idempotency.

## Backend

- [ ] Add domain payment intent, transaction, event, and state transition types.
- [ ] Add DTOs for creating intents, reading intent state, listing transactions, and admin oversight.
- [ ] Add repositories for intent creation, transition updates, transaction append, and event upsert.
- [ ] Add services that validate explicit state transitions and never panic on invalid transitions.
- [ ] Add controllers for authenticated user payment-intent creation, provider payment views, and admin payment oversight.

## Tests

- [ ] Repository tests for event idempotency and transaction append behavior.
- [ ] Service tests for allowed and rejected payment state transitions.
- [ ] Fuzz tests for payment lifecycle transitions.
