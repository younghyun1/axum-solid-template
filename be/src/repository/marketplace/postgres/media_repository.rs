use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::marketplace::{
        enums::{ImageUploadStatus, ImageVisibility},
        media::{Image, NewImage},
    },
    schema::images,
};

pub async fn insert_image(
    conn: &mut AsyncPgConnection,
    new_image: NewImage,
) -> Result<Image, diesel::result::Error> {
    match diesel::insert_into(images::table)
        .values(new_image)
        .returning(Image::as_returning())
        .get_result::<Image>(conn)
        .await
    {
        Ok(image) => Ok(image),
        Err(error) => Err(error),
    }
}

pub async fn complete_image_upload(
    conn: &mut AsyncPgConnection,
    image_id: Uuid,
    public_url: Option<String>,
    now: DateTime<Utc>,
) -> Result<Image, diesel::result::Error> {
    match diesel::update(images::table.filter(images::image_id.eq(image_id)))
        .set((
            images::image_upload_status.eq(ImageUploadStatus::Uploaded),
            images::image_public_url.eq(public_url),
            images::image_updated_at.eq(now),
            images::image_uploaded_at.eq(now),
        ))
        .returning(Image::as_returning())
        .get_result::<Image>(conn)
        .await
    {
        Ok(image) => Ok(image),
        Err(error) => Err(error),
    }
}

pub async fn find_image_by_id(
    conn: &mut AsyncPgConnection,
    image_id: Uuid,
) -> Result<Option<Image>, diesel::result::Error> {
    match images::table
        .filter(images::image_id.eq(image_id))
        .select(Image::as_select())
        .first::<Image>(conn)
        .await
        .optional()
    {
        Ok(image) => Ok(image),
        Err(error) => Err(error),
    }
}

pub async fn list_public_provider_profile_images(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
) -> Result<Vec<Image>, diesel::result::Error> {
    match images::table
        .filter(images::provider_profile_id.eq(provider_profile_id))
        .filter(images::image_upload_status.eq(ImageUploadStatus::Uploaded))
        .filter(images::image_visibility.eq(ImageVisibility::Public))
        .order(images::image_created_at.asc())
        .select(Image::as_select())
        .load::<Image>(conn)
        .await
    {
        Ok(images) => Ok(images),
        Err(error) => Err(error),
    }
}

pub async fn list_provider_profile_images(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
) -> Result<Vec<Image>, diesel::result::Error> {
    match images::table
        .filter(images::provider_profile_id.eq(provider_profile_id))
        .order(images::image_created_at.asc())
        .select(Image::as_select())
        .load::<Image>(conn)
        .await
    {
        Ok(images) => Ok(images),
        Err(error) => Err(error),
    }
}
