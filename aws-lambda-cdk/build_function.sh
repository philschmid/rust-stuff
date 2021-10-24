#!/bin/bash

# cargo build --release --target aarch64-unknown-linux-musl #arm 64
cargo build --release --target x86_64-unknown-linux-musl
cd target/x86_64-unknown-linux-musl/release && mkdir -p lambda && cp bootstrap lambda/