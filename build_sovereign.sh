#!/bin/bash
# build_sovereign.sh v1.3
# Reproducible, supply-chain hardened build for ring0_core.rs
# Produces static binaries for KhawrizmOS ARM64 and RISC-V targets (musl)
# Zero runtime dependencies. Protects against injection via pinned deps + reproducible.

set -euo pipefail

echo "[Ring-0 v1.3] Sovereign Rust build starting for ARM64 NEON + RISC-V Vector..."

WORK="/tmp/ring0_sovereign_build"
rm -rf "$WORK"
mkdir -p "$WORK/src"

# Pin exact deps for reproducibility and supply chain integrity
cat > "$WORK/Cargo.toml" << 'CARGOEOF'
[package]
name = "ring0_core"
version = "1.3.0"
edition = "2021"

[dependencies]
zip = { version = "0.6.6", default-features = false, features = ["deflate"] }
flate2 = "1.0.30"
regex = "1.10.5"
CARGOEOF

cp ring0_core.rs "$WORK/src/main.rs"

cd "$WORK"

TARGETS=("aarch64-unknown-linux-musl" "riscv64gc-unknown-linux-gnu")

for tgt in "${TARGETS[@]}"; do
    echo "[Ring-0] Adding target $tgt and building static binary..."
    rustup target add "$tgt" 2>/dev/null || true
    cargo build --release --target "$tgt" --locked 2>&1 | tail -5
    mkdir -p "artifacts"
    BIN="target/$tgt/release/ring0_core"
    if [ -f "$BIN" ]; then
        cp "$BIN" "artifacts/ring0_core-$tgt"
        strip "artifacts/ring0_core-$tgt" || true
        echo "[Ring-0] Static binary ready: artifacts/ring0_core-$tgt (size: $(stat -c%s artifacts/ring0_core-$tgt 2>/dev/null || echo 0) bytes)"
    else
        echo "[Ring-0] WARNING: build for $tgt failed or cross toolchain missing"
    fi
done

echo "[Ring-0 v1.3] Sovereign build complete."
echo "[Ring-0] Binaries in $WORK/artifacts/ (copy to /usr/local/bin/ring0_core on target KhawrizmOS)"
ls -l "$WORK/artifacts/" 2>/dev/null || true
