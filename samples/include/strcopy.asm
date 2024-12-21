; Arguments
; String length ->          R1
; Source address ->         R2
; Destination address ->    R3
strcopy:
    ; Check whether the string length is zero
    ; If so return
    mov r0, r1
    cmp r0, #0
    bne +30
    ret

    ; Copy character to destination
    mov [r2], r0
    mov [r3], r0

    ; Increment pointers and decrement remaining string length
    inc r2
    inc r3
    dec r1

    ; Move next
    jmp strcopy
