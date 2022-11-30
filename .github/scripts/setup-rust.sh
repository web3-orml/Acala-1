#!/usr/bin/env bash

set -e

PROFILE=${1:-"default"}
TOOLCHAIN=${2:-"nightly-2022-08-05"}

echo "Install rust with profile '$PROFILE' and toolchain '$TOOLCHAIN'"

curl https://sh.rustup.rs -sSf | sh -s -- --profile=$PROFILE -y

$HOME/.cargo/bin/rustup default $TOOLCHAIN

$HOME/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain $TOOLCHAIN

echo "$HOME/.cargo/bin" >> $GITHUB_PATH
