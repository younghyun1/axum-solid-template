use tantivy::schema::{Field, STORED, STRING, Schema, TEXT};

#[derive(Debug, Clone, Copy)]
pub struct MarketplaceSearchFields {
    pub kind: Field,
    pub title: Field,
    pub subtitle: Field,
    pub body: Field,
    pub slug: Field,
    pub url_path: Field,
    pub updated_at: Field,
}

pub fn search_schema() -> (Schema, MarketplaceSearchFields) {
    let mut schema_builder = Schema::builder();
    let kind = schema_builder.add_text_field("kind", STRING | STORED);
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let subtitle = schema_builder.add_text_field("subtitle", TEXT | STORED);
    let body = schema_builder.add_text_field("body", TEXT | STORED);
    let slug = schema_builder.add_text_field("slug", STRING | STORED);
    let url_path = schema_builder.add_text_field("url_path", STRING | STORED);
    let updated_at = schema_builder.add_text_field("updated_at", STRING | STORED);

    (
        schema_builder.build(),
        MarketplaceSearchFields {
            kind,
            title,
            subtitle,
            body,
            slug,
            url_path,
            updated_at,
        },
    )
}
