use crate::core::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::operand::Operand;
use crate::core::register_id::RegisterId;
use crate::cpu::CPU;
use crate::CPUResult;

pub trait MemoryCell {
    fn read(&self) -> CPUResult<u8>;
    fn write(&mut self, cpu: &mut CPU, value: u8) -> CPUResult<()>;
    fn mode(&self) -> InstructionMode;
}

impl MemoryCell for Operand {
    fn read(&self) -> CPUResult<u8> {
        Ok(match self {
            Operand::Literal(value) => *value as u8,
            Operand::Register(_, value) | Operand::ZeroPage(_, value) | Operand::Memory(_, value) => *value,
        })
    }

    fn write(&mut self, cpu: &mut CPU, value: u8) -> CPUResult<()> {
        match self {
            Operand::Literal(_) => Err(Interrupt::IllegalMemory),
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
            Operand::ZeroPage(addr, initial_value) | Operand::Memory(addr, initial_value) => {
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

    fn mode(&self) -> InstructionMode {
        match self {
            Operand::Literal(_) => InstructionMode::Immediate,
            Operand::Register(_, _) => InstructionMode::Implied,
            Operand::ZeroPage(_, _) => InstructionMode::ZeroPage,
            Operand::Memory(_, _) => InstructionMode::Absolute
        }
    }
}