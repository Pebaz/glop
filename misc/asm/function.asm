include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

; entry efi_main

struct fn_u64_add
    x UINT64
    y UINT64
ends


;; You can use dots instead of /
;; a/b/c
a.b.c:
    db 0


some_basic_block:

    ;; Begin to call user-defined function

    ;; Allocate special stack frame for every function call!
    ;; some_stack_frame_thing: db sizeof STACK_FRAME

    ;; Allocate stack frame for local variables and arguments!
    some_struct: db sizeof fn_u64_add

    MOVREL R1, some_struct
    MOVI @R1(fn_u64_add.x), 123
