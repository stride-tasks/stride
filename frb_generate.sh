#!/usr/bin/env sh

flutter_rust_bridge_codegen generate \
    --rust-root ./crates/flutter_bridge \
    --rust-input crate::api \
    --dart-output ./app/lib/bridge \
    --rust-output ./crates/flutter_bridge/src/frb_generated.rs
