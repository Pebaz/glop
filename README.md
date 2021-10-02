# slime

Programming language that compiles to UEFI Bytecode.

### Running

```bash
# Note that drive/UEFI/BOOT/BOOT64.efi must exist for this to boot into it
$ qemu-system-x86_64 -bios OVMF.fd -net none -drive format=raw,file=fat:rw:drive/ -nographic
```
