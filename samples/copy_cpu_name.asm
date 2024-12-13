main:
    and $04500200, $FF ; CPU name length, AND to take only one byte
    mov r1, r0
    mov r2, $00000001 ; Start of the CPU name
    mov r3, $04500200 ; Copy to the start of the RAM
    jsr strcopy

; Arguments
; String length ->          R1
; Source address ->         R2
; Destination address ->    R3
strcopy:
    ; Check whether the string length is zero
    ; If so return
    mov r0, r1
    cmp #0
    bne +2
    ret

    ; Copy character to destination
    ldr r2
    str r3

    ; Increment pointers and decrement remaining string length
    inc r2
    inc r3
    dec r1

    ; Move next
    jmp strcopy
