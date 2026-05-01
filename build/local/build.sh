#!/bin/sh
set -eu

require_command() {
  command_name="$1"
  message="$2"

  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "$message" >&2
    exit 1
  fi
}

require_command docker "docker is required for backend builds but was not found in PATH"
require_command npm "npm is required but was not found in PATH"
require_command zstd "zstd is required for frontend asset compression but was not found in PATH"
require_command gzip "gzip is required for frontend asset compression but was not found in PATH"

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/../.." && pwd)
bin_dir="$repo_root/build/bin"
be_static_dir="$repo_root/be/fe"
docker_image="${RUST_BUILD_IMAGE:-rust:1-bookworm}"
docker_target_dir="$repo_root/build/docker-target"
docker_cargo_home="$repo_root/build/cargo-home"
docker_rustflags="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=znver3"

cd "$repo_root/fe"
if [ -f package-lock.json ]; then
  npm ci
else
  npm install
fi
npm run build

mkdir -p "$be_static_dir"
find "$be_static_dir" -mindepth 1 ! -name ".gitkeep" -exec rm -rf {} +

find ./dist -type f \
  ! -iname "*.png" \
  ! -iname "*.jpg" \
  ! -iname "*.jpeg" \
  ! -iname "*.webp" \
  ! -iname "*.avif" \
  ! -iname "*.gif" \
  ! -iname "*.ico" \
  ! -iname "*.gz" \
  ! -iname "*.zst" \
  -exec sh -c '
    for file do
      gzip -9 -c "$file" > "$file.gz"
      zstd --ultra -22 -q -c "$file" > "$file.zst"
    done
  ' sh {} +

cd "$repo_root/fe/dist"
find . -type f | while IFS= read -r file; do
  target="$be_static_dir/$file"
  mkdir -p "$(dirname "$target")"
  cp "$file" "$target"
done

mkdir -p "$docker_target_dir" "$docker_cargo_home"
docker run --rm \
  --user "$(id -u):$(id -g)" \
  -v "$repo_root:/workspace" \
  -w /workspace/be \
  -e CARGO_HOME=/workspace/build/cargo-home \
  -e CARGO_TARGET_DIR=/workspace/build/docker-target \
  -e RUSTFLAGS="$docker_rustflags" \
  "$docker_image" \
  cargo build --release

mkdir -p "$bin_dir"
cp "$docker_target_dir/release/be" "$bin_dir/be"
echo "Built $bin_dir/be"
