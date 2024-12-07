use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::Operand;
use crate::core::registers::RegisterId;
use crate::CPU;
use crate::CPUResult;

pub trait MemoryCell {
    fn read_word(&self) -> CPUResult<u32>;
    fn write_word(&mut self, cpu: &mut CPU, value: u32) -> CPUResult<()>;
    fn mode(&self) -> Addressing;
}

impl MemoryCell for Operand {
    fn read_word(&self) -> CPUResult<u32> {
        Ok(match self {
            Operand::Literal(value) => *value,
            Operand::Register(_, value) | Operand::Memory(_, value) => *value,
            Operand::Void => return Err(Interrupt::IllegalMemory)
        })
    }

    fn write_word(&mut self, cpu: &mut CPU, value: u32) -> CPUResult<()> {
        match self {
            Operand::Literal(_) | Operand::Void => Err(Interrupt::IllegalMemory),
            Operand::Register(id, initial_value) => {
                match id {
                    RegisterId::R0 => cpu.registers.r0 = value,
                    RegisterId::R1 => cpu.registers.r1 = value,
                    RegisterId::R2 => cpu.registers.r2 = value,
                    RegisterId::R3 => cpu.registers.r3 = value,
                    RegisterId::R4 => cpu.registers.r4 = value,
                    RegisterId::R5 => cpu.registers.r5 = value,
                    RegisterId::R6 => cpu.registers.r6 = value,
                    RegisterId::R7 => cpu.registers.r7 = value,
                    RegisterId::R8 => cpu.registers.r8 = value,
                    RegisterId::R9 => cpu.registers.r9 = value,
                    RegisterId::R10 => cpu.registers.r10 = value,
                    RegisterId::R11 => cpu.registers.r11 = value,
                    RegisterId::R12 => cpu.registers.r12 = value,
                    RegisterId::R13 => cpu.registers.r13 = value,
                    RegisterId::R14 => cpu.registers.r14 = value
                }
                *initial_value = value;
                Ok(())
            },
            Operand::Memory(addr, initial_value) => {
                if (0x0000_0000..0xdfff_ffff).contains(addr) {
                    let bytes = value.to_le_bytes();
                    cpu.memory[(*addr as usize)..(*addr as usize + 4)].copy_from_slice(&bytes);
                    *initial_value = value;
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
            Operand::Memory(_, _) => Addressing::Absolute,
            Operand::Void => Addressing::Implied
        }
    }
}
