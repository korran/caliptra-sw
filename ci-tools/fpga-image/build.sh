#!/bin/bash
set -e
set -x

mkdir -p out

SYSTEM_BOOT_SHA256="714cc0b12607c476672f569b3f996ce8b3446bd05b30bffcd1c772c483923098"
if ! (echo "${SYSTEM_BOOT_SHA256} out/system-boot.tar.gz" | sha256sum -c); then
  curl -o out/system-boot.tar.gz https://people.canonical.com/~platform/images/xilinx/zcu-ubuntu-22.04/iot-limerick-zcu-classic-desktop-2204-x05-2-20221123-58-system-boot.tar.gz
  if ! (echo "${SYSTEM_BOOT_SHA256} out/system-boot.tar.gz" | sha256sum -c); then
    echo "Downloaded system-boot file did not match expected sha256sum".
    exit 1
  fi
fi

# Build the rootfs
if [[ -z "${SKIP_DEBOOTSTRAP}" ]]; then
  (rm -r out/rootfs || true)
  mkdir -p out/rootfs
  debootstrap --include git,curl,ca-certificates,locales,libicu72 --arch arm64 --foreign bookworm out/rootfs
  chroot out/rootfs ln -s /sbin/init init
  chroot out/rootfs /debootstrap/debootstrap --second-stage
  chroot out/rootfs useradd runner --shell /bin/bash --create-home
  chroot out/rootfs mkdir /mnt/root_base
  chroot out/rootfs mkdir /mnt/root_overlay
  chroot out/rootfs mkdir /mnt/new_root

  chroot out/rootfs bash -c 'echo caliptra-fpga > /etc/hostname'
  chroot out/rootfs bash -c 'echo auto end0 > /etc/network/interfaces'
  chroot out/rootfs bash -c 'echo allow-hotplug end0 >> /etc/network/interfaces'
  chroot out/rootfs bash -c 'echo iface end0 inet6 auto >> /etc/network/interfaces'
  chroot out/rootfs bash -c 'echo nameserver 2001:4860:4860::8888 > /etc/resolv.conf'
  chroot out/rootfs bash -c 'echo nameserver 2001:4860:4860::8844 >> /etc/resolv.conf'

  chroot out/rootfs bash -c 'echo root:password | chpasswd'

  trap - EXIT
  chroot out/rootfs bash -c 'su runner -c "cd /home/runner && curl -O -L https://github.com/actions/runner/releases/download/v2.308.0/actions-runner-linux-arm64-2.308.0.tar.gz"'
  chroot out/rootfs bash -c 'su runner -c "cd /home/runner && tar xvzf actions-runner-linux-arm64-2.308.0.tar.gz && rm actions-runner-linux-arm64-2.308.0.tar.gz"'

  cp launch-runner.sh out/rootfs/home/runner/
  chroot out/rootfs chmod 755 /home/runner/launch-runner.sh
  chroot out/rootfs chown runner /home/runner/launch-runner.sh
  cp gha-runner.service out/rootfs/etc/systemd/system/
  chroot out/rootfs systemctl enable gha-runner.service
fi

# Build a squashed filesystem from the rootfs
rm out/rootfs.sqsh || true
sudo mksquashfs out/rootfs out/rootfs.sqsh -comp zstd
bootfs_blocks="$((80000 * 2))"
rootfs_bytes="$(stat --printf="%s" out/rootfs.sqsh)"
rootfs_blocks="$((($rootfs_bytes + 512) / 512))"

# Allocate the disk image
fallocate -l $(((2048 + $bootfs_blocks + $rootfs_blocks + 8) * 512)) out/image.img

# Partition the disk image
cat <<EOF | sfdisk out/image.img
label: dos
label-id: 0x4effe30a
device: image.img
unit: sectors
sector-size: 512

p1 : start=2048, size=${bootfs_blocks}, type=c, bootable
p2 : start=$((2048 + $bootfs_blocks)), size=8, type=83
p3 : start=$((2048 + 8 + $bootfs_blocks)), size=${rootfs_blocks}, type=83
EOF

LOOPBACK_DEV="$(losetup --show -Pf out/image.img)"
function cleanup1 {
  losetup -d ${LOOPBACK_DEV}
}
trap cleanup1 EXIT

# Format bootfs partition (kernel + bootloader stages)
mkfs -t vfat "${LOOPBACK_DEV}p1"

# Mount bootfs partition (from image) for modification
mkdir -p out/bootfs
mount "${LOOPBACK_DEV}p1" out/bootfs

function cleanup2 {
  umount out/bootfs
  cleanup1
}
trap cleanup2 EXIT

# Write bootfs contents
tar xvzf out/system-boot.tar.gz -C out/bootfs

# Replace the u-boot boot script with our own
rm out/bootfs/boot.scr.uimg
mkimage -T script -n "boot script" -C none -d boot.scr out/bootfs/boot.scr.uimg
umount out/bootfs
trap cleanup1 EXIT

# Write the rootfs squashed filesystem to the image partition
dd if=out/rootfs.sqsh of="${LOOPBACK_DEV}p3"

# Write a sentinel value to the configuration partition
echo CONFIG_PARTITION > "${LOOPBACK_DEV}p2"
