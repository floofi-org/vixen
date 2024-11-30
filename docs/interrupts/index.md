# Interrupts and faults on Vixen

The Vixen architecture, like any CPU architecture, has to deal with interrupts (or faults). These are low level events that require interruption in the normal program flow and temporary transfer of control to another piece of code.

Consider the following assembly code:
```asm
main:                   ; Our main program
        add 1, 1        ; Run some arbitrary calculation
        pll X           ; This should cause a stack underflow
        jmp main        ; Otherwise repeat infinitely

.interrupt              ; Define a custom interrupt handler
handle_interrupt:
        jam             ; Force the CPU to halt
```

This code runs some normal operation and then tries to read from the stack. Since the stack is currently empty, it causes an interrupt. The interrupt handler we defined simply causes the CPU to stop processing any more instructions.

## Default behavior

By default, when no interrupt handler is defined, the CPU either stops prematurely and displays an error message (in the case of an emulated system) or resets the system (in the case of a real system).

This behavior can be overridden.

## Handled interrupt behavior

When an interrupt occurs and an interrupt handler is defined (that is, a pointer to handling code is present at `0x00FE`), current CPU status is saved in an internal location and control flow jumps to the interrupt handler. The `R7` register also contains the interrupt ID and (if using the `int` instruction) the last nibble (last 4 bits when reading from left to right) from the `A` register.

The interrupt handler will then do all of its processing, and may or may not return control back to the normal control flow using the `irt` (Interrupt Return) instruction.

If an interrupt occurs within the interrupt handler, this will cause a double fault.

## Double fault behavior

A double fault occurs when an interrupt handler caused another interrupt. In this case, normal program flow is paused and jumps to `0xF0F0`, which is a ROM location, where a double fault handler may be defined.

The code below causes a double fault:

```asm
main:                   ; Our main program
        add 1, 1        ; Run some arbitrary calculation
        pll X           ; This should cause a stack underflow
        jmp main        ; Otherwise repeat infinitely

.interrupt              ; Define a custom interrupt handler
handle_interrupt:
        pll X           ; This should cause another stack underflow
        jam             ; Force the CPU to halt
        
.double_fault
handle_df:
        jam             ; Force the CPU to halt
```

When doing `irt` (Interrupt Return) from a double fault handler, control flow will not jump to the original code, but back to the interrupt handler that caused the double fault.

## Triple fault behavior

A triple fault occurs when a double fault also causes an interrupt. This is a very edge case that should not occur under normal circumstances, and therefore causes the system to reset (or, in the case of an emulator, stop and display an error).

The code below causes a triple fault:

```asm
main:                   ; Our main program
        add 1, 1        ; Run some arbitrary calculation
        pll X           ; This should cause a stack underflow
        jmp main        ; Otherwise repeat infinitely

.interrupt              ; Define a custom interrupt handler
handle_interrupt:
        pll X           ; This should cause another stack underflow
        jam             ; Force the CPU to halt
        
.double_fault           ; Define a custom double fault handler
handle_df:
        pll X           ; This should cause another stack underflow
        jam             ; Force the CPU to halt
```

## Interrupt disabling

There are multiple ways one can disable interrupts temporarily or permanently. The official way is to use the `sei` (Set Interrupt Disable High) and `cli` (Clear Interrupt Disable) to respectively disable and enable interrupts.

The following (unofficial) code allows one to disable interrupts without using the aforementioned instructions:

```asm
main:                   ; Our main program
        add 1, 1        ; Run some arbitrary calculation
        pll X           ; This should cause a stack underflow
        ; But since it is ignored, we end up here
        jmp main        ; Otherwise repeat infinitely

.interrupt              ; Define a custom interrupt handler
.double_fault           ; Define a custom double fault handler
handle_interrupt:
        irt             ; This just does nothing and returns control
                        ; flow, effectively ignoring the interrupt
```

## Interrupt reference

The following table contains a list of all the implemented interrupts. They may not all be used. "Can be disabled" refers to whether the interrupt can be disabled with `sei` or not. Interrupts that cannot be disabled are commonly referred to as Non-Maskable Interrupts (NMIs).

| ID     | Description                                   | Can be disabled |
|--------|-----------------------------------------------|-----------------|
| `0x00` | Real-time clock tick                          | Yes             |
| `0x01` | Asynchronous bus event                        | Yes             |
| `0x02` | General hardware fault                        | No              |
| `0x03` | External hardware interrupt                   | No              |
| `0x10` | Breakpoint hit (`int` instruction)            | No              |
| `0x11` | Illegal instruction                           | Yes*            |
| `0x12` | Illegal memory access/Memory protection fault | No              |
| `0x13` | Divide by zero                                | No              |
| `0x20` | Stack overflow                                | No              |
| `0x21` | Stack overflow                                | No              |
| `0xE0` | User-defined interrupt (`A` ends in `0x0`)    | No              |
| `0xE1` | User-defined interrupt (`A` ends in `0x1`)    | No              |
| `0xE2` | User-defined interrupt (`A` ends in `0x2`)    | No              |
| `0xE3` | User-defined interrupt (`A` ends in `0x3`)    | No              |
| `0xE4` | User-defined interrupt (`A` ends in `0x4`)    | No              |
| `0xE5` | User-defined interrupt (`A` ends in `0x5`)    | No              |
| `0xE6` | User-defined interrupt (`A` ends in `0x6`)    | No              |
| `0xE7` | User-defined interrupt (`A` ends in `0x7`)    | No              |
| `0xE8` | User-defined interrupt (`A` ends in `0x8`)    | No              |
| `0xE9` | User-defined interrupt (`A` ends in `0x9`)    | No              |
| `0xEA` | User-defined interrupt (`A` ends in `0xA`)    | No              |
| `0xEB` | User-defined interrupt (`A` ends in `0xB`)    | No              |
| `0xEC` | User-defined interrupt (`A` ends in `0xC`)    | No              |
| `0xED` | User-defined interrupt (`A` ends in `0xD`)    | No              |
| `0xEE` | User-defined interrupt (`A` ends in `0xE`)    | No              |
| `0xEF` | User-defined interrupt (`A` ends in `0xF`)    | No              |
| `0xFE` | Internal system failure                       | No              |
| `0xFF` | System reset                                  | No              |

\* Invalid instruction when Interrupt Disable is high will cause the instruction to be ignored, which may lead to undefined behavior.
