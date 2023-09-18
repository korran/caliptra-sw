#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: make.sh 2023-05-03-raspios-bullseye-arm64-lite.img.xz"
    exit 1
fi

xz -dkc "$1" > fpgaboss.img
LOOPBACK_DEV="$(losetup --show -Pf fpgaboss.img)"
function cleanup1 {
	echo cleanup1
	losetup -d ${LOOPBACK_DEV}
}
#trap cleanup1 EXIT

losetup -c ${LOOPBACK_DEV}

sudo mkdir -p mnt/p2
mount ${LOOPBACK_DEV}p2 mnt/p2
function cleanup2 {
	echo cleanup2
	umount mnt/p2
	cleanup1
}
#trap cleanup2 EXIT

chroot mnt/p2 apt-get update 
chroot mnt/p2 apt-get upgrade -y
chroot mnt/p2 apt-get install -y libftdi1-dev symlinks
chroot mnt/p2 bash -c 'cd /usr/lib/aarch64-linux-gnu && symlinks -c -r .'
