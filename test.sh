#!/bin/bash

wasm-pack build --target=web

rm -rf ./public/pkg

cp -r ./pkg/ ./public/

./esbuild --servedir=./public
