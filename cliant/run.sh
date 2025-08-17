#!/bin/bash

set -e  # エラー時にスクリプトを停止

cargo run --manifest-path ../code_generator/Cargo.toml ../cliant/config_for_code_generation.txt src/generated_code
echo "✅ Code generation completed!"

# 必要に応じてcliantも実行
# echo "🚀 Running cliant..."
# cargo run
