#!/bin/bash
#
# Usage example:
#     bash fmt.sh
#

DIR=$(dirname $0)
ROOT=$DIR/..

echo "==> Formatting project files..."
rustfmt \
  --config-path $ROOT/.rustfmt.toml \
  $ROOT/**/*.rs

echo "==> Done!"
