#!/bin/bash

set -e

main() {
    cargo build --release
    maelstrom test \
        -w echo \
        --bin ./target/release/vortex \
        --node-count 1 \
        --time-limit 10 
}

main

