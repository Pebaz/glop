# Glop

Programming language that compiles to UEFI Bytecode.

<p align=center>
    <img
        src="misc/screenshots/10-03-21-01 JUMPING TO HELLO WORLD INDEFINITELY.PNG"
        alt="Hello World screenshot"
        width=75%
    >
</p>

### Drawing A Pixel

<p align=center>
    <img
        src="misc/screenshots/11-04-2021-02 GREEN PIXEL.png"
        alt="First green pixel on screen"
        width=75%
    >
</p>

### Assembling

```bash
$ python build.py asm/if.asm
```

### Running

```bash
# Note that drive/UEFI/BOOT/BOOT64.efi must exist for this to boot into it
$ qemu-system-x86_64 -bios OVMF.fd -net none -drive format=raw,file=fat:rw:drive/ -nographic
```

### Natural Indexing CLI Tool

<p align=center>
    <img
        src="misc/Natural Indexing.png"
        alt="Natural indexing tool output"
        width=75%
    >
</p>
