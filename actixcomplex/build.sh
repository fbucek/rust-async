#!/usr/bin/env bash

CURRENT=$(pwd)

# client build
cd frontend

cargo web deploy --target=wasm32-unknown-unknown --release

# server build
cd $CURRENT


mkdir -p backend/static
cp ${CARGO_TARGET_DIR}/deploy/frontend.* $CURRENT/backend/static/


cd $CURRENT/..
cargo run --bin actixcomplex
