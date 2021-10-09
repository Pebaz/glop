; Calling Convention: https://speakerdeck.com/retrage/efi-byte-code-virtual-machine-for-fun-and-profit?slide=7

; https://sudonull.com/post/114032-Organizing-x86-procedure-calls-from-EFI-Byte-Code

; Example 1

;--- Subroutine: EBC/x86 gate for Read MSR ----------------------------------;
; Caller must verify x86 support (IA32 or x64) before call this subroutine,  ;
; but this subroutine differentiate IA32/x64 internally.                     ;
;                                                                            ;
; INPUT:  R1 = Global variables pool base address                            ;
;         R6 = MSR index (same as ECX before RDMSR instruction)              ;
; OUTPUT: R3 = MSR data after Read (same as EDX:EAX after RDMSR instruction) ;
;         R4-R7 can be changed                                               ;
;----------------------------------------------------------------------------;
EBC_Read_MSR:
		XOR64		R7,R7  ; R7=0
		PUSH64		R7  ; Storage for output
		MOVQ		R7,R0  ; Address of storage = stack pointer
		PUSHN		R7  ; Parameter#2 = Output address
		PUSHN		R6  ; Parameter#1 = MSR address
		MOVINW		R7,1,0
		CMPI32WEQ	R7,4  ; R7=4 for 32-bit, R7=8 for 64-bit
		MOVIQW		R7,_IA32_Read_MSR  ; This pointer for IA32 (native width=4)
		JMP8CS		Native_Gate
		MOVIQW		R7,_x64_Read_MSR  ; This pointer for x64 (native width=8)
Native_Gate:	ADD64		R7,R1  ; Add base address = R1
		CALL32EXA	R7
		POPN		R6  ; Remove Parameter#1
		POPN		R7  ; Remove Parameter#2
		POP64		R3  ; Read R3 = Output
		RET
