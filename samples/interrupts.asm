main:                           ; Our main program
        add     r0, #1, #1      ; Run some arbitrary calculation
        pll     r1              ; This should cause a stack underflow
        jmp     #main            ; Otherwise repeat infinitely

.interrupt                      ; Define a custom interrupt handler
handle_interrupt:
        pll     r1              ; This should cause another stack underflow
        jam                     ; Force the CPU to halt

.double_fault                   ; Define a custom double fault handler
handle_df:
        pll     r1              ; This should cause another stack underflow
        jam                     ; Force the CPU to halt
