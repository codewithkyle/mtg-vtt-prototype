#!/bin/bash

curl -fsSL https://esbuild.github.io/dl/v0.21.4 | sh

if ! which wasm-pack >/dev/null; then
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi
