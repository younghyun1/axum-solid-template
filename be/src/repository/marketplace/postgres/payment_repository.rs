use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::marketplace::payments::{
        NewPaymentIntent, NewPaymentTransaction, PaymentIntent, PaymentTransaction,
    },
    schema::{payment_intents, payment_transactions},
};

pub async fn insert_payment_intent(
    conn: &mut AsyncPgConnection,
    new_intent: NewPaymentIntent,
) -> Result<PaymentIntent, diesel::result::Error> {
    match diesel::insert_into(payment_intents::table)
        .values(new_intent)
        .returning(PaymentIntent::as_returning())
        .get_result::<PaymentIntent>(conn)
        .await
    {
        Ok(intent) => Ok(intent),
        Err(error) => Err(error),
    }
}

pub async fn insert_payment_transaction(
    conn: &mut AsyncPgConnection,
    new_transaction: NewPaymentTransaction,
) -> Result<PaymentTransaction, diesel::result::Error> {
    match diesel::insert_into(payment_transactions::table)
        .values(new_transaction)
        .returning(PaymentTransaction::as_returning())
        .get_result::<PaymentTransaction>(conn)
        .await
    {
        Ok(transaction) => Ok(transaction),
        Err(error) => Err(error),
    }
}

pub async fn list_user_payment_intents(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Vec<PaymentIntent>, diesel::result::Error> {
    match payment_intents::table
        .filter(payment_intents::user_id.eq(user_id))
        .order(payment_intents::payment_intent_created_at.desc())
        .select(PaymentIntent::as_select())
        .load::<PaymentIntent>(conn)
        .await
    {
        Ok(intents) => Ok(intents),
        Err(error) => Err(error),
    }
}

pub async fn count_payment_intents(
    conn: &mut AsyncPgConnection,
) -> Result<i64, diesel::result::Error> {
    match payment_intents::table.count().get_result::<i64>(conn).await {
        Ok(count) => Ok(count),
        Err(error) => Err(error),
    }
}
