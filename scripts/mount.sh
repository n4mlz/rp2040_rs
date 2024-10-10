#!/bin/sh

ID=$(lsblk -f | grep RP2 | sed 's/^[^a-zA-Z0-9]*//' | awk '{print $1}')
if [ -z "$ID" ]; then
    echo "RP2 not found"
    exit 1
fi
sudo mkdir -p /media/RPI-RP2
sudo mount -o uid=1000,gid=1000 /dev/${ID} /media/RPI-RP2
elf2uf2-rs $1
cp $1.uf2 /media/RPI-RP2
sudo umount /media/RPI-RP2
