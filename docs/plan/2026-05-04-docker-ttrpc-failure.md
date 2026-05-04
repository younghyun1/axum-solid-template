# Docker TTRPC Failure Investigation

## Checklist

- [x] Locate the command or script invoking docker run.
- [x] Determine whether the failure is caused by repository command syntax or Docker runtime state.
- [x] Patch repository scripts if applicable.
- [x] Provide recovery steps if the host Docker daemon/runtime is at fault.

## Findings

- `build/local/build.sh` builds the frontend, copies compressed assets, builds the Rust builder image, then starts that image with `docker run`.
- A minimal `docker run --rm rust-solid-template-be-builder:nightly-bookworm true` fails with the same TTRPC error, so the repository script is not the cause.
- `journalctl -u docker -u containerd` shows containerd trying to connect to a shim address beginning with protobuf bytes before `unix://...`, which indicates a Docker/containerd/runc shim protocol mismatch or packaging/runtime issue.
- No repository patch is appropriate for the failure. The likely fix is to restart or update/downgrade host Docker/containerd/runc packages.
