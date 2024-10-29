#!/bin/bash

cd "$(dirname "$0")" # cd to script dir

wasm-pack build --target web

output_dir=$(realpath "../frontend/src/wasm_realtime_sws")

cp -R ./pkg/ $output_dir

echo "Wasm compiled to $output_dir"
