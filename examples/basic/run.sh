#!/bin/sh
set -eu

ROFI_PREFIX="${ROFI_PREFIX:-}"

cd "$(dirname "$0")"
cargo build
cd ../..

#DEBUGGER can be e.g. "gdb --args"
ROFI_PLUGIN_PATH=target/debug ${DEBUGGER:-} "$ROFI_PREFIX"/bin/rofi \
	-modi run,plugin-example-basic \
	-show plugin-example-basic \
	"$@"
