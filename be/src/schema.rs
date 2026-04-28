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
        country_flag -> Nullable<Bpchar>,
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
    email_verification_tokens,
    iso_country,
    iso_country_subdivision,
    iso_currency,
    iso_language,
    password_reset_tokens,
    permissions,
    role_permissions,
    roles,
    user_roles,
    users,
);
