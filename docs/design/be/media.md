# Backend Media Design

All uploaded images use one shared `images` table. The table stores metadata only: S3-compatible bucket, object key, optional public URL or CDN path, mime type, byte size, dimensions, checksum, upload status, visibility, timestamps, and typed nullable owner references.

`image_type` determines the valid owner relationship. Check constraints must reject mismatched nullable FKs instead of relying only on service code. Examples: provider profile images attach to provider profile ownership, provider blog images attach to provider blog posts, central blog images attach to central blog posts, and banner images attach to advertisement banners.

Image APIs should model a presigned upload lifecycle:

- create metadata and an upload target
- complete metadata after object upload validation
- attach or reorder where relevant
- update visibility or delete/disable through owner/admin workflows

Services own authorization. Repositories should expose explicit operations by typed IDs and avoid owner-type string polymorphism.

Validation must reject unsupported mime types, impossible dimensions, empty object keys, and byte sizes outside configured limits. Invalid metadata returns structured errors and logs with tracing fields; no panic path is acceptable.
