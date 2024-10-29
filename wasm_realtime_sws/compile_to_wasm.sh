#!/bin/bash

set -eo pipefail

cd "$(dirname "$0")" # cd to script dir

wasm-pack build --target web

output_dir=$(realpath "../frontend/src/wasm_realtime_sws")

cp -R ./pkg/ $output_dir
rm $output_dir/.gitignore # We do want to track the wasm files
rm $output_dir/README.md  # The README is just a copy of the one here

echo "Wasm compiled to $output_dir"
