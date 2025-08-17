#!/bin/bash

set -e  # エラー時にスクリプトを停止

echo "🚀 Running code_generator..."
cargo run --manifest-path ../code_generator/Cargo.toml ../cliant/config_for_code_generation.txt src/generated_code
echo "✅ Code generation completed!"

echo "🚀 Running tests..."
cargo test
echo "✅ Test completed!"

if [ $1!="-t" ]; then
    echo "🚀 Running cliant..."
    cargo run
fi

# 必要に応じてcliantも実行
# echo "🚀 Running cliant..."
# cargo run
