use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};

use crate::{
    controller::v1::marketplace::{
        admin_active_bans, admin_clear_marketplace_public_cache, admin_create_ban,
        admin_create_banner, admin_create_central_blog_post, admin_marketplace_overview,
        admin_reindex_marketplace_search, admin_revoke_ban, marketplace_active_banners,
        marketplace_search, provider_complete_image_upload, provider_create_blog_post,
        provider_create_image, provider_get_profile, provider_upsert_profile,
        public_provider_blog_post, public_provider_detail, public_provider_directory,
        user_create_payment_intent, user_get_profile, user_list_payment_intents,
        user_upsert_profile,
    },
    init::state::server_state::ServerState,
    middleware::auth::require_auth,
};

pub fn build_public_marketplace_router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/marketplace/search", get(marketplace_search))
        .route("/marketplace/providers", get(public_provider_directory))
        .route("/marketplace/providers/{slug}", get(public_provider_detail))
        .route(
            "/marketplace/providers/{provider_slug}/blog/{post_slug}",
            get(public_provider_blog_post),
        )
        .route(
            "/marketplace/banners/{placement}",
            get(marketplace_active_banners),
        )
}

pub fn build_protected_marketplace_router(state: Arc<ServerState>) -> Router<Arc<ServerState>> {
    Router::new()
        .route(
            "/marketplace/user/profile",
            get(user_get_profile).post(user_upsert_profile),
        )
        .route(
            "/marketplace/payments/intents",
            get(user_list_payment_intents).post(user_create_payment_intent),
        )
        .route(
            "/marketplace/provider/profile",
            get(provider_get_profile).post(provider_upsert_profile),
        )
        .route(
            "/marketplace/provider/blog",
            post(provider_create_blog_post),
        )
        .route("/marketplace/provider/images", post(provider_create_image))
        .route(
            "/marketplace/provider/images/{image_id}/complete",
            post(provider_complete_image_upload),
        )
        .route(
            "/marketplace/admin/overview",
            get(admin_marketplace_overview),
        )
        .route(
            "/marketplace/admin/search/reindex",
            post(admin_reindex_marketplace_search),
        )
        .route(
            "/marketplace/admin/cache/clear",
            post(admin_clear_marketplace_public_cache),
        )
        .route("/marketplace/admin/bans/active", get(admin_active_bans))
        .route("/marketplace/admin/bans", post(admin_create_ban))
        .route(
            "/marketplace/admin/bans/{ban_id}/revoke",
            post(admin_revoke_ban),
        )
        .route(
            "/marketplace/admin/blog",
            post(admin_create_central_blog_post),
        )
        .route("/marketplace/admin/banners", post(admin_create_banner))
        .layer(from_fn_with_state(state, require_auth))
}
