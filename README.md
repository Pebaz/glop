# Glop

Programming language that compiles to UEFI Bytecode.

### Drawing A Gradient Square

Simple algorithm to draw a square:

```rust
@clear-screen(),

let x = 0,
let y = 64,

@draw-pixel(1, 1, 55, 255, 0),
@draw-pixel(64, 1, 55, 255, 0),
@draw-pixel(128, 1, 55, 255, 0),

@draw-pixel(1, 1, 55, 255, 0),
@draw-pixel(1, 64, 55, 255, 0),
@draw-pixel(1, 128, 55, 255, 0),

loop
[
    if @u64-gte(y, 128)
    [
        @draw-pixel(8, 8, 255, 0, 55),
        break,
    ]
    else
    [
        loop
        [
            if @u64-gte(x, 64)
            [
                @draw-pixel(10, 10, 255, 0, 55),
                set x = 0,
                break,
            ]
            else
            [
                @draw-pixel(
                    x,
                    y,
                    @u64-add(100, x),
                    y,
                    @u64-add(x, y),
                ),
            ]

            set x = @u64-add(x, 1),
        ]
    ]

    set y = @u64-add(y, 1),
]
```

Compile it with:

```
$ glop lang/goal3.glp drive\EFI\BOOT\BOOTX64.efi
```

Run it with:

```bash
# Manually:
$ qemu-system-x86_64 -bios misc/OVMF.fd -net none -drive format=raw,file=fat:rw:drive/

# Windows PowerShell:
$ ./run.ps1
```

Result when run:

<p align=center>
    <img
        src="misc/screenshots/12-07-2021-01 FINISHED PROJECT - GLOP PROGRAM RUNNING CORRECTLY .png"
        alt="Hello world program with purple square with gradient"
        width=75%
    >
</p>

### Drawing Hello World Without A Newline

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
