# A folder `drive/` needs to exist
# Additionally, a `drive/EFI/BOOT/BOOTX64.efi` can be used to directly run a bootloader
# Add `-nographic` to run in the terminal
qemu-system-x86_64 -bios misc/OVMF.fd -net none -drive format=raw,file=fat:rw:drive/
