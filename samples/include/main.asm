main:
    and $00000000, #$FF ; CPU name length, AND to take only one byte
    mov r1, r0
    ldr #$00000001 ; Start of the CPU name
    mov r2, r0
    ldr #$04500200 ; Copy to the start of the RAM
    mov r3, r0
    jsr strcopy
    int

.include "strcopy.asm"
