Backend:
For the Rust codebase, Use of `unwrap()` and `expect()` is not tolerable. All Option<T> and Result<T, E> types must be dealt explicitly through match statements only (occasionally if lets and such if clippy complains). Generated code will not include any pathways to explicit panics if at all possible; all errors will be logged and a graceful exit from the current context will be executed if an exit has to be made.
In projects using `tracing`, info, warning, and error messages must be logged in a structured manner in this way, using % or ? or whatever as necessary:
```rust
error!(env_key = key, error = %e, "Missing required environment variable");
```
Performance is key to Rust. When using `diesel` and other database frameworks exclude N+1 errors. Avoid unnecessary copies. Prefer scc or Tokio RwLocks over mutexes when possible. For blocking tasks within Tokio, use spawn_blocking.
Run `cargo fmt`, and `cargo clippy` after finishing large refactorings and fix generated warnings and errors.
Use of Linux intrinsics using `libc` and such are encouraged when more performant. We develop for modern versions of Linux only.
We'll be building for a single-server on-prem deployment; no Valkey or Redis. In-memory caches are important in that regard.
Files should be modularized when they get above 300 LOC.
All folders should have a mod.rs containing nothing but module declarations.
All API requests and responses must be costructed in /be/src/dto.
All domain objects must be in /be/src/domain.
API and DB conventions should be checked before implementing any new APIs or database models, in terms of response construction, schema alteration, etc.
I prefer UUIDv7 PKEYs, and for all column names to be prefixed by the table name; user_id instead of id, for example.
Database models should make extensive use of newtyping using the 'nutype' library.
Docs:
Document all design conventions in /docs/design/be, /docs/design/fe, etc.
Document all API using utopia and utoipa swagger UI.
Comments:
Commentary should be terse and technical.
Unit tests:
Implement unit tests wherever possible.
Scripts:
One-time use scripts should be produced as Rust binaries in the same workspace as /be/ as a separate binary for maximum integration.
DB schema design:
For the DB, make rich use of enums, diesel_cli, and its migration system. 
Building:
Don't do release mode builds and waste time.
FE:
For the SolidJS project - we're using TypeScript, not JS. Strict, strict, strict typing at all times.
Tailwind CSS, latest Vite, keep it SOTA and performant.
Note that we will be serving the built and gzipped/zstd'd version (do both at maximum compression) from the BE server itself.
Styling and classes should be highly centralized, not applied haphazardly case by case. Consistent visual style and modularized elements!