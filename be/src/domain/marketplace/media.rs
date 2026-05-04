use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::images;

use super::enums::{ImageType, ImageUploadStatus, ImageVisibility};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    pub image_id: Uuid,
    pub image_type: ImageType,
    pub image_upload_status: ImageUploadStatus,
    pub image_visibility: ImageVisibility,
    pub image_bucket: String,
    pub image_object_key: String,
    pub image_public_url: Option<String>,
    pub image_mime_type: String,
    pub image_byte_size: i64,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub image_checksum_sha256: Option<String>,
    pub user_id: Option<Uuid>,
    pub provider_profile_id: Option<Uuid>,
    pub provider_blog_post_id: Option<Uuid>,
    pub central_blog_post_id: Option<Uuid>,
    pub advertisement_banner_id: Option<Uuid>,
    pub image_created_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
    pub image_uploaded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub image_type: ImageType,
    pub image_upload_status: ImageUploadStatus,
    pub image_visibility: ImageVisibility,
    pub image_bucket: String,
    pub image_object_key: String,
    pub image_public_url: Option<String>,
    pub image_mime_type: String,
    pub image_byte_size: i64,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub image_checksum_sha256: Option<String>,
    pub user_id: Option<Uuid>,
    pub provider_profile_id: Option<Uuid>,
    pub provider_blog_post_id: Option<Uuid>,
    pub central_blog_post_id: Option<Uuid>,
    pub advertisement_banner_id: Option<Uuid>,
}
