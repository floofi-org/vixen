main:
    and $04500200, $FF ; CPU name length, AND to take only one byte
    mov r1, r0
    mov r2, $00000001 ; Start of the CPU name
    mov r3, $04500200 ; Copy to the start of the RAM
    jsr strcopy

.include "strcopy.asm"
