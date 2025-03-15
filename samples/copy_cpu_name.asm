main:
    and r1, $00000000, #$FF ; CPU name length, AND to take only one byte
    mov r2, #$00000001 ; Start of the CPU name
    mov r3, #$04500208 ; Copy to the start of the RAM
    jmp strcopy
    int

; Arguments
; String length ->          R1
; Source address ->         R2
; Destination address ->    R3
strcopy:
    ; Check whether the string length is zero
    ; If so return
    cmp r1, #0
    jnz copy
    ret

    copy:
        ; Copy character to destination
        and [r3], [r2], #$FF

        ; Increment pointers and decrement remaining string length
        inc r2
        inc r3
        dec r1

        ; Move next
        jmpl strcopy
