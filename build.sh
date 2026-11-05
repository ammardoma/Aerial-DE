#!/usr/bin/env bash
set -e

echo "=== Building Aurora Desktop Environment (Release) ==="
cargo build --release --workspace

echo "=== Aurora Desktop Binaries Built Successfully ==="
ls -lh target/release/aurora-* | grep -v "\.d$"

PREFIX="${PREFIX:-/usr/local}"
if [ "$1" = "install" ]; then
    echo "=== Installing Aurora DE to ${PREFIX}/bin ==="
    install -d "${PREFIX}/bin"
    install -m 755 target/release/aurora-wm "${PREFIX}/bin/"
    install -m 755 target/release/aurora-shell "${PREFIX}/bin/"
    install -m 755 target/release/aurora-settings "${PREFIX}/bin/"
    install -m 755 target/release/aurora-session "${PREFIX}/bin/"
    install -m 755 target/release/aurora-msg "${PREFIX}/bin/"
    echo "Installation complete."
fi
