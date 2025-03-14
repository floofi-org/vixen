main:                           ; Our main program
        add     r0, #1, #1      ; Run some arbitrary calculation
        pop     r1              ; This should cause a stack underflow
        jmpl    main            ; Otherwise repeat infinitely

.interrupt                      ; Define a custom interrupt handler
handle_interrupt:
        pop     r1              ; This should cause another stack underflow
        hlt                     ; Force the CPU to halt

.double_fault                   ; Define a custom double fault handler
handle_df:
        pop     r1              ; This should cause another stack underflow
        hlt                     ; Force the CPU to halt
