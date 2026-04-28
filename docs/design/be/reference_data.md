# Backend Reference Data

ISO country, language, currency, and subdivision tables are loaded into `ReferenceDataCache` during backend startup.

- Public customer-facing metadata is served from in-memory cache, not queried per request.
- Current API endpoints:
  - `GET /api/v1/reference/countries`
  - `GET /api/v1/reference/languages`
  - `GET /api/v1/reference/countries/{country_code}/subdivisions`
- Reference API responses use DTOs from `/be/src/dto/reference_data.rs`.
- Country responses expose `country_flag` and `country_primary_language`; the frontend uses these for country labels and primary-language ordering.
- Subdivision responses include the parent `country_flag` so UI labels can stay consistent without an extra lookup.

Reference endpoints are read-only and public. The source tables remain database-backed startup seed data, but request-time access must remain cache-only unless the mutability model changes.
