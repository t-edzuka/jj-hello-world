# Enabling stable features like `just --fmt`

set unstable := true

build:
    cargo build

alias f := format

format:
    just --fmt
    cargo fmt
