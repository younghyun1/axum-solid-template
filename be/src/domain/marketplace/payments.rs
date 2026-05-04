use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde_json::Value;
use uuid::Uuid;

use crate::schema::{payment_intents, payment_processor_events, payment_transactions};

use super::enums::{
    PaymentIntentStatus, PaymentProvider, PaymentTransactionKind, PaymentTransactionStatus,
    ProcessorEventStatus,
};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = payment_intents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentIntent {
    pub payment_intent_id: Uuid,
    pub user_id: Uuid,
    pub provider_profile_id: Uuid,
    pub payment_intent_amount_minor_units: i64,
    pub payment_intent_currency: i32,
    pub payment_provider: PaymentProvider,
    pub payment_intent_status: PaymentIntentStatus,
    pub payment_intent_processor_reference: Option<String>,
    pub payment_intent_metadata: Value,
    pub payment_intent_created_at: DateTime<Utc>,
    pub payment_intent_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = payment_intents)]
pub struct NewPaymentIntent {
    pub user_id: Uuid,
    pub provider_profile_id: Uuid,
    pub payment_intent_amount_minor_units: i64,
    pub payment_intent_currency: i32,
    pub payment_provider: PaymentProvider,
    pub payment_intent_status: PaymentIntentStatus,
    pub payment_intent_metadata: Value,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = payment_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentTransaction {
    pub payment_transaction_id: Uuid,
    pub payment_intent_id: Uuid,
    pub payment_transaction_kind: PaymentTransactionKind,
    pub payment_transaction_status: PaymentTransactionStatus,
    pub payment_transaction_amount_minor_units: i64,
    pub payment_transaction_currency: i32,
    pub payment_transaction_processor_reference: Option<String>,
    pub payment_transaction_created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = payment_transactions)]
pub struct NewPaymentTransaction {
    pub payment_intent_id: Uuid,
    pub payment_transaction_kind: PaymentTransactionKind,
    pub payment_transaction_status: PaymentTransactionStatus,
    pub payment_transaction_amount_minor_units: i64,
    pub payment_transaction_currency: i32,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = payment_processor_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentProcessorEvent {
    pub payment_processor_event_id: Uuid,
    pub payment_provider: PaymentProvider,
    pub payment_processor_event_external_id: String,
    pub payment_processor_event_status: ProcessorEventStatus,
    pub payment_processor_event_payload: Value,
    pub payment_processor_event_processed_at: Option<DateTime<Utc>>,
    pub payment_processor_event_created_at: DateTime<Utc>,
}
