# Backend Payments Design

Payments are represented as an internal ledger and payment-intent lifecycle. The backend should not hard-code a processor such as Stripe or Adyen in domain or DTO names.

Core records:

- payment intents: requested payment with user, provider, amount, currency, state, processor adapter, processor reference, and metadata
- transactions: immutable ledger entries for authorized, captured, refunded, failed, or adjusted movements
- processor events: idempotent external event records with raw payload, processing status, and timestamps

State transitions must be explicit and validated in service code. Invalid transitions return domain errors. The repository layer should expose compare-and-update operations where concurrent webhook or admin actions may race.

All money amounts use integer minor units plus ISO currency code. DTOs should not accept floating-point money values.

Provider-specific integration code, when added, belongs behind a payment adapter boundary. Public API responses expose internal state and safe processor display metadata only.
