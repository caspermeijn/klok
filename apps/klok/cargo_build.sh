#!/bin/bash
set -eu
# Copyright (C) 2020 Casper Meijn <casper@meijn.net>
#
# SPDX-License-Identifier: CC0-1.0

TARGET="thumbv7m-none-eabi"
BUILD_PROFILE="${MYNEWT_BUILD_PROFILE-default}"

if [ "${BUILD_PROFILE}" = "optimized" ] || [ "${BUILD_PROFILE}" = "speed" ]; then
  cargo +nightly build --release --target="${TARGET}" --target-dir="${MYNEWT_PKG_BIN_DIR}"
  cp "${MYNEWT_PKG_BIN_DIR}"/${TARGET}/release/*.a "${MYNEWT_PKG_BIN_ARCHIVE}"
else
  cargo +nightly build --target="${TARGET}" --target-dir="${MYNEWT_PKG_BIN_DIR}"
  cp "${MYNEWT_PKG_BIN_DIR}"/${TARGET}/debug/*.a "${MYNEWT_PKG_BIN_ARCHIVE}"
fi
