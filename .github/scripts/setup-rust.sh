#!/usr/bin/env bash

set -e

curl https://sh.rustup.rs -sSf | sh -s -- --profile=$(or $(profile),default) -y

source "$HOME/.cargo/env"

rustup default $(or $(toolchain),nightly-2022-08-05)

rustup target add wasm32-unknown-unknown --toolchain $(or $(toolchain),nightly-2022-08-05)

echo "$HOME/.cargo/bin" >> $GITHUB_PATH
