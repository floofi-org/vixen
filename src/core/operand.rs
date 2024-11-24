use crate::core::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::register_id::RegisterId;
use crate::cpu::CPU;
use crate::CPUResult;

#[derive(Debug)]
pub enum Operand<'a> {
    Literal(u16),
    Register(RegisterId, &'a u8),
    ZeroPage(u16, &'a u8),
    Memory(u16, &'a u8)
}

impl<'a> Operand<'a> {
    pub fn decode(raw_operand: u16, cpu: &CPU, mode: InstructionMode) -> CPUResult<Operand> {
        match &mode {
            InstructionMode::Immediate => Ok(Operand::Literal(raw_operand)),
            InstructionMode::Implied => {
                let register = RegisterId::try_from(raw_operand)?;
                Ok(Operand::Register(register, cpu.get_register(register)))
            },
            InstructionMode::ZeroPage => {
                if raw_operand > 0xFF {
                    Err(Interrupt::IllegalMemory)
                } else {
                    let operand = Operand::ZeroPage(raw_operand, &cpu.memory[raw_operand as usize]);
                    Ok(operand)
                }
            },
            InstructionMode::Absolute => {
                if raw_operand <= 0xFF {
                    Err(Interrupt::IllegalMemory)
                } else {
                    let operand = Operand::Memory(raw_operand, &cpu.memory[raw_operand as usize]);
                    Ok(operand)
                }
            }
        }
    }
}