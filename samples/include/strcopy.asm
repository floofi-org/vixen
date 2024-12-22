; Arguments
; String length ->          R1
; Source address ->         R2
; Destination address ->    R3
strcopy:
    ; Check whether the string length is zero
    ; If so return
    cmp r1, #0
    bne #copy
    ret

    copy:
        ; Copy character to destination
        and [r3], [r2], #$FF

        ; Increment pointers and decrement remaining string length
        inc r2
        inc r3
        dec r1

        ; Move next
        jmp #strcopy
