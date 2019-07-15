#!/bin/bash
cargo watch -s "cargo clippy && cargo build && RUST_BACKTRACE=1 cargo test && RUST_BACKTRACE=1 cargo run"