use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::Operand;
use crate::core::registers::RegisterId;
use crate::CPU;
use crate::CPUResult;

pub trait MemoryCell {
    fn read_word(&self) -> CPUResult<u8>;
    fn read_dword(&self) -> CPUResult<u16>;
    fn write_word(&mut self, cpu: &mut CPU, value: u8) -> CPUResult<()>;
    fn write_dword(&mut self, cpu: &mut CPU, value: u16) -> CPUResult<()>;
    fn mode(&self) -> Addressing;
}

impl MemoryCell for Operand {
    fn read_word(&self) -> CPUResult<u8> {
        Ok(match self {
            #[allow(clippy::cast_possible_truncation)]
            // Literals should only be 8-bit
            Operand::Literal(value) => *value as u8,
            Operand::Register(_, value) | Operand::ZeroPage(_, value, _) | Operand::Memory(_, value, _) => *value,
            Operand::Void => return Err(Interrupt::IllegalMemory)
        })
    }

    fn read_dword(&self) -> CPUResult<u16> {
        Ok(match self {
            Operand::Literal(value) => *value,
            Operand::Register(_, value) => u16::from(*value),
            Operand::ZeroPage(_, high_value, low_value) |
                Operand::Memory(_, high_value, low_value) =>
                    u16::from_le_bytes([*high_value, *low_value]),
            Operand::Void => return Err(Interrupt::IllegalMemory)
        })
    }

    fn write_word(&mut self, cpu: &mut CPU, value: u8) -> CPUResult<()> {
        match self {
            Operand::Literal(_) | Operand::Void => Err(Interrupt::IllegalMemory),
            Operand::Register(id, initial_value) => {
                match id {
                    RegisterId::A => cpu.registers.a = value,
                    RegisterId::X => cpu.registers.x = value,
                    RegisterId::Y => cpu.registers.y = value,
                    RegisterId::R0 => cpu.registers.r0 = value,
                    RegisterId::R1 => cpu.registers.r1 = value,
                    RegisterId::R2 => cpu.registers.r2 = value,
                    RegisterId::R3 => cpu.registers.r3 = value,
                    RegisterId::R4 => cpu.registers.r4 = value,
                    RegisterId::R5 => cpu.registers.r5 = value,
                    RegisterId::R6 => cpu.registers.r6 = value,
                    RegisterId::R7 => cpu.registers.r7 = value
                }
                *initial_value = value;
                Ok(())
            },
            Operand::ZeroPage(addr, initial_value,_) | Operand::Memory(addr, initial_value, _) => {
                if (0x0200..0xDFFF).contains(addr) {
                    cpu.memory[*addr as usize] = value;
                    *initial_value = value;
                    Ok(())
                } else {
                    Err(Interrupt::IllegalMemory)
                }
            }
        }
    }

    fn write_dword(&mut self, cpu: &mut CPU, value: u16) -> CPUResult<()> {
        match self {
            Operand::Literal(_) | Operand::Void => Err(Interrupt::IllegalMemory),
            #[allow(clippy::cast_possible_truncation)]
            // Registers are 8-bit and cannot have 16-bit data written to them
            Operand::Register(id, initial_value) => {
                match id {
                    RegisterId::A => cpu.registers.a = value as u8,
                    RegisterId::X => cpu.registers.x = value as u8,
                    RegisterId::Y => cpu.registers.y = value as u8,
                    RegisterId::R0 => cpu.registers.r0 = value as u8,
                    RegisterId::R1 => cpu.registers.r1 = value as u8,
                    RegisterId::R2 => cpu.registers.r2 = value as u8,
                    RegisterId::R3 => cpu.registers.r3 = value as u8,
                    RegisterId::R4 => cpu.registers.r4 = value as u8,
                    RegisterId::R5 => cpu.registers.r5 = value as u8,
                    RegisterId::R6 => cpu.registers.r6 = value as u8,
                    RegisterId::R7 => cpu.registers.r7 = value as u8
                }
                *initial_value = value as u8;
                Ok(())
            },
            Operand::ZeroPage(addr, initial_high_value, initial_low_value) |
                Operand::Memory(addr, initial_high_value, initial_low_value) => {
                if (0x0200..0xDFFE).contains(addr) {
                    let bytes = addr.to_le_bytes();
                    cpu.memory[*addr as usize] = bytes[0];
                    cpu.memory[*addr as usize + 1] = bytes[1];
                    *initial_high_value = bytes[0];
                    *initial_low_value = bytes[1];
                    Ok(())
                } else {
                    Err(Interrupt::IllegalMemory)
                }
            }
        }
    }

    fn mode(&self) -> Addressing {
        match self {
            Operand::Literal(_) => Addressing::Immediate,
            Operand::Register(_, _) => Addressing::Direct,
            Operand::ZeroPage(_, _, _) => Addressing::ZeroPage,
            Operand::Memory(_, _, _) => Addressing::Absolute,
            Operand::Void => Addressing::Implied
        }
    }
}
