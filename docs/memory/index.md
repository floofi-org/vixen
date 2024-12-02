# Memory management and registers on Vixen

The Vixen architecture contains 2 types of memory: registers and virtual memory. Registers are used for immediate processing within the CPU itself, and virtual memory is used for the rest.

Vixen currently supports up to **64 kibibytes (65536 bytes)** of virtual memory.

## Registers

The following table lists all the registers, the internal ID used to refer to them, how large they are, and whether they are readable and/or writeable.

| Name | Internal ID       | Size    | Description                                                                                       | Readable | Writable |
|------|-------------------|---------|---------------------------------------------------------------------------------------------------|----------|----------|
| `A`  | `0x0001`          | 8 bits  | Accumulator register, used for arithmetic results                                                 | x        | x        |
| `X`  | `0x0011`          | 8 bits  | Index register 1, for loops and memory indexing                                                   | x        | x        |
| `Y`  | `0x0012`          | 8 bits  | Index register 1, for loops and memory indexing                                                   | x        | x        |
| `R0` | `0x1000`          | 8 bits  | General-purpose register 1                                                                        | x        | x        |
| `R1` | `0x1001`          | 8 bits  | General-purpose register 2                                                                        | x        | x        |
| `R2` | `0x1002`          | 8 bits  | General-purpose register 3                                                                        | x        | x        |
| `R3` | `0x1003`          | 8 bits  | General-purpose register 4                                                                        | x        | x        |
| `R4` | `0x1004`          | 8 bits  | General-purpose register 5                                                                        | x        | x        |
| `R5` | `0x1005`          | 8 bits  | General-purpose register 6                                                                        | x        | x        |
| `R6` | `0x1006`          | 8 bits  | General-purpose register 7                                                                        | x        | x        |
| `R7` | `0x1007`          | 8 bits  | General-purpose register 8                                                                        | x        | x        |
| `PC` | `program_counter` | 16 bits | Address of current instruction in memory                                                          | x        | x        |
| `SR` | `system_register` | 16 bits | Status register: zero, carry, overflow, interrupt/break, double fault and interrupt disable flags | *        | *        |
| `UP` | `stack_pointer`   | 8 bits  | Pointer to the top of the user stack                                                              | *        | *        |
| `SS` | `system_stack`    | -       | Internal stack used for interrupt handling and subroutines                                        |          |          |
| `SP` | -                 | -       | Pointer to the top of the system stack                                                            |          |          |

\* There are instructions that might allow for reading/writing to these registers, but they cannot be directly read from/written to.

## Memory map

Since Vixen is using virtual memory, all the memory the CPU sees may not necessarily correspond to system RAM. Not all memory regions are writable, and they are all readable.

| Start    | End      | Size (B) | Name                                       | Description                                                                                                                                                                                                                      |
|----------|----------|----------|--------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `0x0000` | `0x00FF` | 256      | Zero Page                                  | The zero page is similar to CPU cache and is designed for fast read and write. Instruction that make use of the zero page are specific but perform better than accessing the rest of the memory.                                 |
| `0x0100` | `0x01FF` | 256      | User Stack                                 | The user stack is used for various stack-related operations. It can be used through the PSH and PLL instructions (for example) without conflict with the CPU itself.                                                             |
| `0x0200` | `0xCFFF` | 51.5K    | RAM                                        | RAM is used to store disorganized memory. It is slower than the zero page and than registers but used to off-load data that does not need immediate processing. Only about 52 KiB can be used as RAM as per architecture design. |
| `0xD000` | `0xDFFF` | 4K       | System Bus                                 | This I/O region is used to access the system bus and send or receive information from/to external devices such as terminals, non-volative memory, sound cards or printers.                                                       |
| `0xE000` | `0xFE00` | 8K       | Boot ROM                                   | ROM contains the system's basic code, which is usually used to load another external program (bootloader). It is usually stored in a dedicated chip and can (in most cases) not be written to.                                   |
| `0xFE00` | `0xFFFF` | 512      | [Specification](../specification/index.md) | The CPU specification region contains read-only information about the CPU, such as its model number, clock speed, bus speed, supported extensions, bus width, address with, and more.                                            |

Only memory from `0x0000` to `0xDFFF` is natively writeable. Bus devices may also request for their memory to be locked.

## Addressing modes

There are multiple ways one can read or write data. This mode is defined in nibble 0 (the last 4 bits when reading left to right) when encoding operation codes:

| Operation code | Name      | Description                                                               | Assembly syntax           |
|----------------|-----------|---------------------------------------------------------------------------|---------------------------|
| `0x...0`       | Immediate | Uses literal values or no values at all.                                  | `#<value>`                |
| `0x...1`       | Direct    | Uses a register within the CPU.                                           | `<name>`                  |
| `0x...2`       | Zero Page | Uses a memory location from the zero page (`0x0000` to `0x00FF`)          | `<address>`               |
| `0x...3`       | Absolute  | Uses an absolute memory location starting from `0x0000`                   | `<address>`               |
| `0x...4`       | Relative  | Uses a memory location relative to the current location (program counter) | `<offset>` or `-<offset>` |
| `0x...5`       | Implied   | Uses no operands or implicit operands                                     | -                         |

In assembly code, numbers can be in different formats:

| Syntax | Base                  | Example     |
|--------|-----------------------|-------------|
| `%`    | Binary (base 2)       | `%10101010` |
| `$`    | Hexadecimal (base 16) | `$16EF`     |
| -      | Decimal (base 10)     | `123`       |
