# UK-Centric Provider Directory: ISO Subdivision Refactor

Date: 2026-05-17
Status: in progress, **`cargo check` red — see "Resume here"**.

---

## Pickup context

- **Repo root:** `/home/cyh/Personal/2026/rust-solid-template`
- **Branch:** `main` (uncommitted; do NOT commit until check + tests green)
- **DB:** Postgres via UNIX socket. `DATABASE_URL` for the `diesel` CLI:
  ```
  postgres://svc_admin:fh4u293gf74gu9%21@/svc_db?host=/run/postgresql
  ```
  (`!` URL-escaped as `%21`. Full creds in `be/.env`.)
- **Migration ALREADY APPLIED** to the live `svc_db`. `diesel migration list`
  shows `[X] 2026-05-17-000000-0000_provider_profile_subdivision`.
- **Reference HTML at `./index.html`** (root) = mood-board only; ignore its
  industry-specific copy. UK-centric service. Generic "Provider Exchange" brand
  is the live theme.
- **Browsers installed for render verification:** `google-chrome-stable`
  (148.x), `firefox` (150.x). Headless screenshot recipe at bottom.

### Git diff (uncommitted)

```
 M be/src/build_info.rs                 (do not touch; auto-rewritten by build.rs)
 M be/src/domain/marketplace/provider.rs
 M be/src/dto/marketplace/mapper.rs
 M be/src/dto/marketplace/request.rs
 M be/src/dto/marketplace/response.rs
 M be/src/init/state/cache/marketplace/key.rs
 M be/src/repository/marketplace/postgres/provider_repository.rs
 M be/src/schema.rs                     (regen from diesel print-schema, missing QueryId derives)
 M be/src/service/marketplace/provider.rs
 M be/src/service/marketplace/public.rs
 M fe/src/app/AppHeader.tsx             (PARTIAL — see warning below)
?? be/migrations/2026-05-17-000000-0000_provider_profile_subdivision/{up,down}.sql
?? docs/plan/2026-05-17-uk-subdivision-directory.md   (this file)
```

### PARTIAL EDIT WARNING — `fe/src/app/AppHeader.tsx`

Top of file currently has:
- `For` imported but unused.
- `UK_CITY_LINKS` constant declared but unused.
- `onCitySelect: (area: string) => void` added to `AppHeaderProps` but no caller
  in `App.tsx` passes it and the header body never reads it.

Effect: TS build fails. Vite dev (no typecheck) renders fine. Either finish the
wiring (FE plan below) or revert these three additions before running
`npm run build`.

---

## Goal

Replace free-text `provider_profile_service_area` with FK to
`iso_country_subdivision`. UK-centric front-end: major-city quick links on the
immersive intro bar, plus the existing detailed-search dropdown populated with
the full GB ISO 3166-2 list.

## Why

- Free-text = no canonical join, no flag/grouping, no clean filter equality.
- `iso_country_subdivision` already seeded (5 048 rows, see
  `be/migrations/2026-04-28-005000-0000_country_subdivisions_insert/up.sql`).
- `users.user_subdivision` already uses this FK — providers should mirror it.

---

## Resume here (cargo check failures)

`cargo check` in `be/` fails with **18 errors** of the form:

```
error[E0277]: the trait bound `schema::sql_types::ModerationStatus: QueryId` is not satisfied
   --> src/repository/marketplace/postgres/provider_repository.rs:186:10
   --> src/schema.rs:38:5
```

Same error fires for every custom SQL enum: `ModerationStatus`,
`ProviderProfileStatus`, `BlogPostStatus`, `BannerPlacement`, `BannerStatus`,
`ImageType`, `ImageUploadStatus`, `ImageVisibility`. Hit sites:
- `repository/marketplace/postgres/provider_repository.rs:186, 317, 341, 363`
- `repository/marketplace/postgres/admin_repository.rs:101, 140, 195` (plus more)
- `init/state/cache/reference_data/load.rs:125` (different shape — a
  `CompatibleType` failure that likely cascades from the enum issue)

**Root cause:** the diesel CLI regen I ran (`diesel print-schema > src/schema.rs`)
emitted the enum struct definitions as:

```rust
#[derive(diesel::sql_types::SqlType)]
#[diesel(postgres_type(name = "moderation_status"))]
pub struct ModerationStatus;
```

without `QueryId`. Pre-regen schema (from git HEAD) had the QueryId derive on
every enum. Diesel CLI requires either a `diesel.toml` with
`custom_type_derives` or a CLI flag to re-emit.

**Fix path (pick one — option A preferred):**

A. Create `be/diesel.toml`:
```toml
[print_schema]
file = "src/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId", "Clone"]

[migrations_directory]
dir = "migrations"
```
Then:
```bash
cd /home/cyh/Personal/2026/rust-solid-template/be
DATABASE_URL='postgres://svc_admin:fh4u293gf74gu9%21@/svc_db?host=/run/postgresql' \
  diesel print-schema > src/schema.rs
cargo check --color never 2>&1 | tail -20
```

B. Hand-edit each `pub struct EnumName;` in `be/src/schema.rs` (lines 5–62) to:
```rust
#[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
```

C. Sanity-restore via git: `git show HEAD:be/src/schema.rs > /tmp/old_schema.rs`,
diff against the regen to spot any unintended joinable! / table column drift
beyond the `provider_profile_subdivision_id` addition + `provider_profile_service_area`
removal.

---

## Done so far

### Migration: `be/migrations/2026-05-17-000000-0000_provider_profile_subdivision`

`up.sql`:
```sql
ALTER TABLE public.provider_profiles
    ADD COLUMN provider_profile_subdivision_id INTEGER,
    ADD CONSTRAINT fk_provider_profiles_subdivision
        FOREIGN KEY (provider_profile_subdivision_id)
        REFERENCES public.iso_country_subdivision (subdivision_id)
        ON DELETE SET NULL;
DROP INDEX IF EXISTS public.idx_provider_profiles_service_area;
ALTER TABLE public.provider_profiles DROP COLUMN provider_profile_service_area;
CREATE INDEX idx_provider_profiles_subdivision
    ON public.provider_profiles (provider_profile_subdivision_id);
```

`down.sql`: drops the index + FK + column, re-adds the TEXT column + old index.
Pre-launch — no data-preservation step.

**Already applied** to live DB. Re-running `diesel migration run` will be a no-op.

### Schema (`be/src/schema.rs`)

- Regenerated via `diesel print-schema`. New column landed:
  `provider_profile_subdivision_id -> Nullable<Int4>`.
- New joinable: `diesel::joinable!(provider_profiles -> iso_country_subdivision
  (provider_profile_subdivision_id));`
- **BROKEN:** missing `QueryId` derive on enums (see Resume section).

### Domain (`be/src/domain/marketplace/provider.rs`)

- `ProviderProfile`, `NewProviderProfile`, `ProviderProfileUpdate`: removed
  `provider_profile_service_area: Option<String>`; added
  `provider_profile_subdivision_id: Option<i32>`.

### DTOs (`be/src/dto/marketplace/{request,response,mapper}.rs`)

- `ProviderDirectoryQuery`: `service_area` → `subdivision_id: Option<i32>` +
  `subdivision_code: Option<String>` (composite `GB-LND` form for query-string
  ergonomics from the FE).
- `UpsertProviderProfileRequest`: `service_area` → `subdivision_id: Option<i32>`.
- `ProviderProfileResponse`: `service_area: Option<String>` → `subdivision:
  Option<ProviderSubdivisionResponse>`.
- `ProviderDirectoryCardResponse`: same swap.
- New `ProviderSubdivisionResponse { subdivision_id, country_code,
  country_alpha2, subdivision_code, subdivision_name, subdivision_type }`.
- Mapper `From<ProviderProfile>` sets `subdivision: None`; services hydrate it.

### Cache key (`be/src/init/state/cache/marketplace/key.rs`)

```rust
pub fn provider_directory(
    query: Option<&str>,
    subdivision_id: Option<i32>,
    subdivision_code: Option<&str>,
    limit: i64,
) -> String { ... "public:providers:q={}:sid={}:scode={}:limit={}" ... }
```
Test `provider_directory_key_normalizes_query_values` updated to:
```
provider_directory(Some("  Roofing "), Some(2826), Some(" GB-LND "), 24)
== "public:providers:q=roofing:sid=2826:scode=gb-lnd:limit=24"
```

### Repository (`be/src/repository/marketplace/postgres/provider_repository.rs`)

- New `SubdivisionWithCountry { subdivision, country_alpha2 }`.
- New `load_subdivisions_with_country(conn, &[i32])` →
  `HashMap<i32, SubdivisionWithCountry>` via `iso_country_subdivision INNER JOIN
  iso_country ON country_code`.
- New `find_subdivision_id_by_code(composite_code: &str) -> Option<i32>` — splits
  on `-`, upper-cases both halves, filters by `country_alpha2 + subdivision_code`.
  Required because subdivision codes collide across countries
  (`BIR` exists for GB, RU, etc.).
- `list_public_providers(q, subdivision_id, limit)` — drop `ilike`, use
  `provider_profile_subdivision_id.eq(value)`.
- `update_provider_profile_by_user` writes `provider_profile_subdivision_id`.

### Services

- `service/marketplace/public.rs`:
  - `provider_directory` resolves `subdivision_code → id` when only the composite
    form arrived; short-circuits with an empty list (cached) if the code is
    unknown.
  - Hydrates `subdivision` on every card using `load_subdivisions_with_country`.
  - `provider_detail` hydrates the single profile's `subdivision` field.
  - Cache key call updated to new 4-arg signature.
  - Local `fn subdivision_response(&SubdivisionWithCountry) ->
    ProviderSubdivisionResponse` helper.
- `service/marketplace/provider.rs`:
  - `upsert_provider_profile`: validation block now just reads `request.subdivision_id`
    (no `validation::short_optional` for it).
  - `provider_profile` (provider self-view) hydrates `subdivision` the same way.
  - Inlines the `ProviderSubdivisionResponse` mapping (duplicated from `public.rs`
    — fine for now; if a third caller appears, lift it to `dto::marketplace::mapper`).

---

## Still pending

### Front-end (none of this is started; AppHeader change is half-done — see warning)

Files that reference `service_area` and need swap to `subdivision`:
- `fe/src/api/marketplaceTypes.ts` (lines ~52, 65, 140) — replace
  `service_area: string | null` with
  `subdivision: ProviderSubdivisionResponse | null` (mirror BE shape).
- `fe/src/api/marketplaceApi.ts` line 29: function signature for
  `getProviderDirectory({ q?, subdivision_id?, subdivision_code? })`.
- `fe/src/app/pages/providerDirectoryModel.ts` lines 9, 23, 51 + tests in
  `providerDirectoryModel.test.ts` line 84.
- `fe/src/app/pages/ProviderDirectoryPage.tsx` lines 33, 65 (`filters` shape).
- `fe/src/app/pages/ProviderListingGrid.tsx` line 81 (`provider.service_area`
  display).
- `fe/src/app/pages/ProviderDetailPage.tsx` lines 53, 89.
- `fe/src/app/pages/ProviderDashboardPage.tsx` line 48 (upsert payload).

### Front-end (new flows)

- `AppHeader.tsx` — finish the half-done edit:
  - Render `<nav class="city-links">` in immersive mode left slot using
    `UK_CITY_LINKS`.
  - Each link calls `props.onCitySelect(cityLabel)` (or pass code directly —
    cleaner: pass `{ name, code }` pair so the URL gets `GB-LND` not `London`).
- `App.tsx` — add `onCitySelect={(code) => navigate(\`/providers?subdivision=${code}\`)}`.
- `ProviderDirectoryPage` — read `useLocation().search` for `subdivision=…`,
  seed `subdivision_code` filter, group results into headings per major city
  with a "more areas" detailed-search dropdown beneath.
- `getCountrySubdivisions` (already exists in `fe/src/api/appApi.ts:51`) — call
  with the GB country code (resolve via `getCountries` lookup, alpha2 == "GB")
  to populate the detailed-search dropdown.

### CSS

- `fe/src/styles/navigation.css` — add `.city-links` rules (small text, flex row,
  rgba white on the immersive overlay, hover full-white).
- `fe/src/styles/template-directory.css` — section headings per major area.

### Tests

- `cargo test -p be` — at minimum the cache-key test now passes; backend
  integration tests likely need updating where they build `ProviderDirectoryQuery`
  or assert `service_area`.
- `npm test --prefix fe` — `providerDirectoryModel.test.ts` needs the new
  `subdivision` shape.

---

## Major UK subdivision quick-links (verify against seed)

Confirmed in `be/migrations/2026-04-28-005000-0000_country_subdivisions_insert/up.sql`:

| City        | ISO 3166-2 | Notes                                   |
| ----------- | ---------- | --------------------------------------- |
| Birmingham  | `GB-BIR`   | Metropolitan district                   |
| Bristol     | `GB-BST`   | "City of Bristol", unitary authority    |
| Belfast     | `GB-BFS`   | District council area (NI)              |

Not yet verified in the seed (skim only saw the first ~50 GB rows):
- `GB-LND` (London) — likely "City of London" + the `GB-LDN` borough cluster.
  **Pick which one represents "London" for the UX before wiring.**
- `GB-MAN` (Manchester) — confirm. May only have `BOL`/`BUR`/etc. boroughs of
  Greater Manchester rather than a single Manchester entry.
- `GB-LDS` (Leeds), `GB-EDH` (Edinburgh), `GB-GLG` (Glasgow) — confirm with:
  ```bash
  grep "country_alpha2 = 'GB'" \
    be/migrations/2026-04-28-005000-0000_country_subdivisions_insert/up.sql \
    | grep -iE "London|Manchester|Leeds|Edinburgh|Glasgow"
  ```

If a major city has no single subdivision row, the quick link should target a
"London (all boroughs)" pseudo-filter — easiest version: store a hard-coded
`Vec<subdivision_code>` per quick-link on the FE and send the first match; or
extend BE `subdivision_code` query param to accept a comma-separated list.
Decide before wiring.

---

## Repro / verify

```bash
# Backend compile
cd /home/cyh/Personal/2026/rust-solid-template/be
cargo check --color never 2>&1 | tail -20

# Migration state
DATABASE_URL='postgres://svc_admin:fh4u293gf74gu9%21@/svc_db?host=/run/postgresql' \
  diesel migration list

# Front-end dev (logs to /tmp/render-shots/dev.log; PID printed)
cd /home/cyh/Personal/2026/rust-solid-template/fe
npm run dev > /tmp/render-shots/dev.log 2>&1 &

# Headless screenshot
mkdir -p /tmp/render-shots
google-chrome-stable --headless=new --hide-scrollbars --no-sandbox \
  --window-size=1440,900 \
  --screenshot=/tmp/render-shots/home.png http://127.0.0.1:5173/

# Cache key unit test
cargo test --color never -p be -- provider_directory_key_normalizes_query_values
```

## Don't-do list

- Don't `git push` or commit until `cargo check` + `npm run build` clean.
- Don't touch `be/src/build_info.rs` (auto-rewritten by `build.rs` on every cargo
  invocation; the diff is build-id churn).
- Don't re-run `diesel migration run` expecting reseed — migration is idempotent;
  the column is already in the live DB.
- Don't widen `find_subdivision_id_by_code` to match by code alone — codes
  collide across countries.
