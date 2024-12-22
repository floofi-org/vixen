main:
    and r1, $00000000, #$FF ; CPU name length, AND to take only one byte
    mov r2, #$00000001 ; Start of the CPU name
    mov r3, #$04500200 ; Copy to the start of the RAM
    jsr #strcopy
    int

.include "strcopy.asm"
