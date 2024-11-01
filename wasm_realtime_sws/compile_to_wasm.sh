#!/bin/bash

set -eo pipefail

cd "$(dirname "$0")" # cd to script dir

wasm-pack build --target web

# Hack: Add an import at the top of the generated JS file.
# This is needed for WASM/AudioWorklet reasons, see
# https://github.com/rustwasm/wasm-pack/issues/689
# or https://www.toptal.com/webassembly/webassembly-rust-tutorial-web-audio
cp ./polyfill/TextEncoder.js ./pkg/TextEncoder.js
temp_file=$(mktemp)
echo "import './TextEncoder.js'" >"$temp_file"
cat "./pkg/wasm_audio.js" >>"$temp_file"
mv "$temp_file" "./pkg/wasm_audio.js"

output_dir=$(realpath "../frontend/src/wasm_realtime_sws")
cp -R ./pkg/ $output_dir
rm $output_dir/.gitignore # We do want to track the wasm files
rm $output_dir/README.md  # The README is just a copy of the one here

echo "Wasm compiled to $output_dir"
