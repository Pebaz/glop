<p align=center>
    <img src="misc/Glop.png" alt="Glop Programming Language Logo" width=60%>
</p>

# Glop

> Programming language that compiles to UEFI Bytecode.

### Drawing A Gradient Square

Simple algorithm to draw a square:

```zig
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
$ glop hello-world.glp drive/EFI/BOOT/BOOTX64.efi
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
    >
</p>

## Installation

This project relies upon the Flat Assembler G. You can download it from
[here](https://flatassembler.net/download.php), *but make sure you install
`fasmg` and not `fasm`*. It's the one at the bottom of the page above `FASMARM`
which is a distribution of fasm but for ARM processors. Once downloaded, unzip
in a directory of your choice and add that directory to your PATH so that
`fasmg-ebc-rs` can find it at runtime.

```
$ cargo install --git https://github.com/Pebaz/Glop
```

## Running Compiled Glop Programs

To run programs written in Glop, you could:

1. Use QEMU
2. Boot into it on a UEFI compatible machine

To use QEMU, simply use the bundled `misc/OVMF.fd` stub to run the EFI Bytecode
program:

```bash
# Create a virtual drive folder structure understood by EFI for QEMU:
$ mkdir drive/EFI/BOOT/

# Generate the executable
$ glop hello-world.glp drive/EFI/BOOT/BOOTX64.efi

# Run it!
$ qemu-system-x86_64 -bios misc/OVMF.fd -net none -drive format=raw,file=fat:rw:drive/
```

Alternatively, to run Glop programs on real hardware, add this folder structure
to a removable drive:

```bash
/EFI/BOOT/BOOTX64.efi  # The compiled Glop program
```

This folder structure is understood by EFI and will be selected to boot into
when the removable drive is chosen as the boot option. Please note that your
computer must support UEFI in order to run Glop programs directly.

## Documentation

Glop is a very simple language and only supports a couple features:

* `@clear-screen(),` intrinsic
* `@u64-gte(a, b),` intrinsic
* `@u64-eq(a, b),` intrinsic
* `@u64-add(a, b),` intrinsic
* `draw-pixel(x, y, r, g, b)` intrinsic
* `loop` statement
* `break` statement
* `if/else` statement (must include `else` block even if not using it)
* `let` statement
* `set` statement
* Unsigned 64 bit integer literals (u64)

Statements are terminated with a comma, but commas are not needed after a
block (`[..]`). Comments are not supported at this time. Variables must be
declared with `let` like: `let number = 123,` and can be optionally updated
later with `set number = @u64-add(number, 1),`.

---

# Development Log

Why does this project exist? I was learning about operating systems and then I
discovered that every UEFI compliant computer has a bytecode virtual machine
that is capable of running cross-platform software before an OS is loaded. I'm
actually really surprised there are not more compilers that target EFI Bytecode
but I understand it's a very fringe target (it doesn't even support floats!).

My journey to understand UEFI Bytecode led me to create [Spore](https://github.com/Pebaz/Spore),
a disassembler for UEFI Bytecode. In the process of creating Glop, I finally
learned assembly language and "get it" now. It is vastly interesting and I look
forward to working more with it.

Glop is definitely in a barely finished minimum-viable-product state as of
right now (12/13/21). I hope to come back to it and add more features but in
the meantime, the existing feature set is enough to write small programs that
draw things on screen. Additionally, if desired, many more compiler intrinsics
could be added that would allow pretty cool new features.

### Drawing Hello World Without A Newline

<p align=center>
    <img
        src="misc/screenshots/10-03-21-01 JUMPING TO HELLO WORLD INDEFINITELY.PNG"
        alt="Hello World screenshot"
    >
</p>

### Drawing A Pixel

<p align=center>
    <img
        src="misc/screenshots/11-04-2021-02 GREEN PIXEL.png"
        alt="First green pixel on screen"
    >
</p>

### Natural Indexing CLI Tool

<p align=center>
    <img
        src="misc/Natural Indexing.png"
        alt="Natural indexing tool output"
    >
</p>

## Stargazers over time

[![Stargazers over time](https://starchart.cc/Pebaz/Glop.svg)](https://starchart.cc/Pebaz/Glop)
