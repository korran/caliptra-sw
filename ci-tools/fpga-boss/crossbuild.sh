#!/bin/bash

PKG_CONFIG_SYSROOT_DIR=$PWD/image/mnt/p2 cargo build --target aarch64-unknown-linux-gnu --release
