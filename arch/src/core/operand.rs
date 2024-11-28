use alloc::format;
use alloc::string::String;
use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::registers::register_id::RegisterId;
use crate::cpu::CPU;
use crate::CPUResult;

#[derive(Debug)]
pub enum Operand {
    Literal(u16),
    Register(RegisterId, u8),
    ZeroPage(u16, u8, u8),
    Memory(u16, u8, u8)
}

impl Operand {
    pub fn decode(raw_operand: u16, cpu: &CPU, mode: InstructionMode) -> CPUResult<Operand> {
        match &mode {
            InstructionMode::Immediate => Ok(Operand::Literal(raw_operand)),
            InstructionMode::Implied => Self::implied(raw_operand, cpu),
            InstructionMode::ZeroPage => Self::zero_page(raw_operand, cpu),
            InstructionMode::Absolute => Self::memory(raw_operand, cpu),
            InstructionMode::Relative => {
                let offset = raw_operand as i16;
                let target = if offset > 0 {
                    cpu.program_counter + offset.unsigned_abs()
                } else {
                    cpu.program_counter - offset.unsigned_abs()
                };
                if target < 0xFF {
                    Err(Interrupt::IllegalMemory)
                } else {
                    Self::memory(target, cpu)
                }
            },
        }
    }

    fn implied(register: u16, cpu: &CPU) -> CPUResult<Self> {
        let register = RegisterId::try_from(register)?;
        let value = cpu.get_register(register);

        Ok(Operand::Register(register, value))
    }

    fn zero_page(address: u16, cpu: &CPU) -> CPUResult<Self> {
        if address > 0xFF {
            return Err(Interrupt::IllegalMemory);
        }

        let high_value = cpu.memory[address as usize];
        let low_value = cpu.memory[address as usize + 1];
        Ok(Operand::ZeroPage(address, high_value, low_value))
    }

    fn memory(address: u16, cpu: &CPU) -> CPUResult<Self> {
        if address <= 0xFF {
            return Err(Interrupt::IllegalMemory);
        }

        let high_value = cpu.memory[address as usize];
        let low_value = cpu.memory[address as usize + 1];
        Ok(Operand::Memory(address, high_value, low_value))
    }

    pub fn disassemble(raw_operand: u16, cpu: &CPU, mode: InstructionMode) -> String {
        if let Ok(operand) = Operand::decode(raw_operand, cpu, mode) {
            operand.disassemble_self()
        } else {
            format!("??({raw_operand:0>4X})")
        }
    }

    pub fn disassemble_self(&self) -> String {
        match self {
            Self::Literal(value) => format!("#${value:X}"),
            Self::Register(id, _) => format!("{id:?}"),
            Self::ZeroPage(address, _, _) => format!("${address:0>2X}"),
            Self::Memory(address, _, _) => format!("${address:0>4X}"),
        }
    }
}
