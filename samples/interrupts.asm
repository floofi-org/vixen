main:                           ; Our main program
        add     #1, #1          ; Run some arbitrary calculation
        pll     R1              ; This should cause a stack underflow
        jmp     main            ; Otherwise repeat infinitely

.interrupt                      ; Define a custom interrupt handler
handle_interrupt:
        pll     R1              ; This should cause another stack underflow
        jam                     ; Force the CPU to halt

.double_fault                   ; Define a custom double fault handler
handle_df:
        pll     R1              ; This should cause another stack underflow
        jam                     ; Force the CPU to halt
