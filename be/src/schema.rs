pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_run_status"))]
    pub struct JobRunStatus;
}

diesel::table! {
    auth_refresh_sessions (auth_refresh_session_id) {
        auth_refresh_session_id -> Uuid,
        user_id -> Uuid,
        auth_refresh_session_token_hash -> Varchar,
        auth_refresh_session_created_at -> Timestamptz,
        auth_refresh_session_expires_at -> Timestamptz,
        auth_refresh_session_last_used_at -> Nullable<Timestamptz>,
        auth_refresh_session_rotated_at -> Nullable<Timestamptz>,
        auth_refresh_session_revoked_at -> Nullable<Timestamptz>,
        auth_refresh_session_user_auth_token_version -> Int4,
    }
}

diesel::table! {
    email_verification_tokens (email_verification_token_id) {
        email_verification_token_id -> Uuid,
        user_id -> Uuid,
        email_verification_token -> Uuid,
        email_verification_token_expires_at -> Timestamptz,
        email_verification_token_created_at -> Timestamptz,
        email_verification_token_used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    iso_country (country_code) {
        country_code -> Int4,
        country_alpha2 -> Bpchar,
        country_alpha3 -> Bpchar,
        country_eng_name -> Varchar,
        country_primary_language -> Int4,
        country_currency -> Int4,
        phone_prefix -> Varchar,
        country_flag -> Bpchar,
        is_country -> Bool,
    }
}

diesel::table! {
    iso_country_subdivision (subdivision_id) {
        subdivision_id -> Int4,
        country_code -> Int4,
        subdivision_code -> Varchar,
        subdivision_name -> Varchar,
        subdivision_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    iso_currency (currency_code) {
        currency_code -> Int4,
        currency_alpha3 -> Bpchar,
        currency_name -> Varchar,
    }
}

diesel::table! {
    iso_language (language_code) {
        language_code -> Int4,
        language_alpha2 -> Bpchar,
        language_alpha3 -> Bpchar,
        language_eng_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JobRunStatus;

    job_runs (job_run_id) {
        job_run_id -> Uuid,
        job_run_name -> Text,
        job_run_status -> JobRunStatus,
        job_run_scheduled_for -> Timestamptz,
        job_run_started_at -> Nullable<Timestamptz>,
        job_run_finished_at -> Nullable<Timestamptz>,
        job_run_duration_ms -> Nullable<Int8>,
        job_run_attempt -> Int4,
        job_run_error_code -> Nullable<Text>,
        job_run_error_message -> Nullable<Text>,
        job_run_metadata -> Jsonb,
        job_run_created_at -> Timestamptz,
        job_run_updated_at -> Timestamptz,
    }
}

diesel::table! {
    password_reset_tokens (password_reset_token_id) {
        password_reset_token_id -> Uuid,
        user_id -> Uuid,
        password_reset_token -> Uuid,
        password_reset_token_expires_at -> Timestamptz,
        password_reset_token_created_at -> Timestamptz,
        password_reset_token_used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    permissions (permission_id) {
        permission_id -> Uuid,
        permission_name -> Text,
        permission_description -> Nullable<Text>,
    }
}

diesel::table! {
    role_permissions (role_permission_id) {
        role_permission_id -> Uuid,
        role_id -> Uuid,
        permission_id -> Uuid,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Uuid,
        role_name -> Text,
        role_description -> Nullable<Text>,
    }
}

diesel::table! {
    user_roles (user_role_id) {
        user_role_id -> Uuid,
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        user_name -> Varchar,
        user_email -> Varchar,
        user_password_hash -> Varchar,
        user_created_at -> Timestamptz,
        user_updated_at -> Timestamptz,
        user_password_changed_at -> Timestamptz,
        user_last_login_at -> Nullable<Timestamptz>,
        user_is_email_verified -> Bool,
        user_auth_token_version -> Int4,
        user_country -> Int4,
        user_language -> Int4,
        user_subdivision -> Nullable<Int4>,
    }
}

diesel::joinable!(auth_refresh_sessions -> users (user_id));
diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(iso_country -> iso_currency (country_currency));
diesel::joinable!(iso_country -> iso_language (country_primary_language));
diesel::joinable!(iso_country_subdivision -> iso_country (country_code));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(users -> iso_country (user_country));
diesel::joinable!(users -> iso_country_subdivision (user_subdivision));
diesel::joinable!(users -> iso_language (user_language));

diesel::allow_tables_to_appear_in_same_query!(
    auth_refresh_sessions,
    email_verification_tokens,
    iso_country,
    iso_country_subdivision,
    iso_currency,
    iso_language,
    job_runs,
    password_reset_tokens,
    permissions,
    role_permissions,
    roles,
    user_roles,
    users,
);
