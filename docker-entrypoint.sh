#!/bin/sh

diesel migration run --config-file domain/diesel.toml
cargo run
