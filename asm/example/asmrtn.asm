;; This file contains all of the assembly routines available to a Glop progtam
;; running in the UEFI environment. They are presented logically as
;; instructions in their own right but they are in fact routines.

;; ------------------------------- Instructions -------------------------------
;; These are actual, valid instructions that are called using the example
;; `ASMCALL`

;; Pop 2 u64s off the stack and push their sum.
ADDU64:
    POP R2
    POP R1
    ADD R1, R2
    PUSH R1
    JMP32 R6(+0, +2)


;; --------------------------- Pseudo-Instructions ----------------------------
;; These are instructions that cannot or should not be expressed as a logical
;; instruction (assembly routine). For instance, re-implementing STACKPUSH
;; would just be a waste as every platform has ways to push onto the stack.
;; Don't actually jump to any of the labels in this section. They are meant for
;; copying and pasting most of the time.

;; Example of how to push a u64 to the stack
PUSHU64:
    MOVI R1, 123
    PUSH R1

;; How to call an assembly routine.
ASMCALL:
    ;; PUSH arguments to the stack in reverse order (right to left)
    MOVI R1, 123      ;; Arg1
    PUSH R1
    MOVI R1, 1        ;; Arg0
    PUSH R1
    STORESP R6, [IP]  ;; ASMCALL ADDU64
    JMP ADDU64
