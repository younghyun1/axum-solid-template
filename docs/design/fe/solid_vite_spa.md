# Solid Vite SPA Conventions

The frontend lives in `/fe` as a strict TypeScript Solid application built by Vite.

- Source files stay under `/fe/src`.
- Shared API envelope types, normalized backend error handling, and typed customer-facing API calls stay under `/fe/src/api`.
- Styling is centralized under `/fe/src/styles`; components should use named classes rather than ad hoc inline styling.
- The primary app surface is customer-facing: account creation, sign-in, profile, recovery, verification, and service status. Raw endpoint/demo UI should stay out of the default customer flow.
- Light and dark themes are controlled through semantic CSS variables on `:root` and `:root[data-theme="dark"]`.
- Page transitions should stay fast, around 120-160ms, and respect `prefers-reduced-motion`.
- The local Vite dev server proxies `/api` to the backend on `127.0.0.1:3000`.
- `npm run build` runs `tsc --noEmit` before `vite build`.
- `/build/local/build.sh` installs frontend dependencies, builds the SPA, creates gzip and zstd variants for compressible assets, copies the complete output into `/be/fe`, builds the backend with `cargo build --release`, and copies the final binary into `/build/bin`.
- `/be/fe` is generated backend embed input and is ignored except for `.gitkeep`.

Production binaries embed `/be/fe` through `rust_embed`; use `/build/local/build.sh` when producing the local deployable binary.
