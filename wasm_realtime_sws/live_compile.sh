#!/bin/bash

cd "$(dirname "$0")" # cd to script dir

fswatch src/ --include '.rs$' | while read -r changed_file; do
    clear
    echo "Change detected in: $changed_file"
    ./compile_to_wasm.sh
done
