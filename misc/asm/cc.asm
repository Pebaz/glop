include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

;; This is for all the assembly for the whole program
section 'CODE' code executable readable

TESTCALLINGCONVENTION:
    PUSHn R6(0, +6)

    ;; Touching native code MUST touch R6 at some point.
    MOVREL R1, string_a
    PUSH R1
    CALL __print
    POP R1

    POPn R6
    JMP32 R6

__print:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    ; CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R1, @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    CALLEX    R1
    MOV       R0, R0(+2,0)
    RET

efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL R1, system_table
    MOVn @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)


    STORESP R6, [IP]
    ; JMP TESTCALLINGCONVENTION

    ;; 81 10 FA FE FF FF  JMP32 R0 -262  ;; Relative Address
    JMP32 R0(TESTCALLINGCONVENTION)

    STORESP R6, [IP]
;; JMP
JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)


    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)
    STORESP R6, [IP]
    ;; JMP TESTCALLINGCONVENTIONJMP
    JMP32 R0(TESTCALLINGCONVENTION)

    ; RET
    ; RET

    MOVREL R1, temporary_string_status
    PUSH R1
    CALL __print
    POP R1

    loop_forever:
        JMP loop_forever
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?
    temporary_string_status: du "<HERE>", 0x0D, 0x0A, 0x00
    string_a: du "<a>", 0x0D, 0x0A, 0x00
    string_b: du "<b>", 0x0D, 0x0A, 0x00
    string_c: du "<c>", 0x0D, 0x0A, 0x00
    string_d: du "<d>", 0x0D, 0x0A, 0x00

;; This is for initialized global variables
section 'DATA' data readable writeable
    const_u64_0: dq 64
