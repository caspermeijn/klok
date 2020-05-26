#!/bin/bash
set -eu
# Copyright (C) 2020 Casper Meijn <casper@meijn.net>
#
# SPDX-License-Identifier: CC0-1.0

TARGET="thumbv7m-none-eabi"
cargo build --target="${TARGET}" --target-dir="${MYNEWT_PKG_BIN_DIR}"
cp "${MYNEWT_PKG_BIN_DIR}"/${TARGET}/debug/*.a "${MYNEWT_PKG_BIN_ARCHIVE}"