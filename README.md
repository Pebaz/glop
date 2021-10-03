# slime

Programming language that compiles to UEFI Bytecode.

<img src="misc/screenshots/10-03-21-01 JUMPING TO HELLO WORLD INDEFINITELY.PNG">

### Running

```bash
# Note that drive/UEFI/BOOT/BOOT64.efi must exist for this to boot into it
$ qemu-system-x86_64 -bios OVMF.fd -net none -drive format=raw,file=fat:rw:drive/ -nographic
```
