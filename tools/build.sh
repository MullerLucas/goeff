#!/bin/env bash

echo "-------------------------------------------"

SH_DIR=$(dirname "$0")

./$SH_DIR/parse_styles || exit 1

echo "-------------------------------------------"

cargo build --all-features || exit 1
