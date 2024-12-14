use alloc::format;
use alloc::string::String;
use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::registers::RegisterId;
use crate::CPU;
use crate::CPUResult;

#[derive(Debug)]
pub enum Operand {
    Literal(u32),
    Register(RegisterId, u32),
    Memory(u32, u32),
    Void
}

impl Operand {
    pub fn decode(raw_operand: u32, cpu: &CPU, mode: Addressing) -> CPUResult<Operand> {
        match &mode {
            Addressing::Immediate => Ok(Self::Literal(raw_operand)),
            Addressing::Direct => Self::direct(raw_operand, cpu),
            Addressing::Absolute => Self::memory(raw_operand, cpu),
            Addressing::Relative => {
                // Relative offsets are stored as u32 internally
                // but need to be interpreted as i32.
                #[allow(clippy::cast_possible_wrap)]
                let offset = raw_operand as i32;
                let target = if offset > 0 {
                    cpu.program_counter + offset.unsigned_abs()
                } else {
                    cpu.program_counter - offset.unsigned_abs()
                };
                #[allow(clippy::cast_possible_truncation)]
                if target > (cpu.memory.len() - 4) as u32 {
                    Err(Interrupt::IllegalMemory)
                } else {
                    Self::memory(target, cpu)
                }
            },
            Addressing::RegisterIndirect => {
                let register = RegisterId::try_from(raw_operand)?;
                let target = cpu.get_register(register);
                #[allow(clippy::cast_possible_truncation)]
                if target > (cpu.memory.len() - 4) as u32 {
                    Err(Interrupt::IllegalMemory)
                } else {
                    Self::memory(target, cpu)
                }
            },
            Addressing::Indirect => {
                let target = u32::from_le_bytes([
                    cpu.memory[raw_operand as usize], cpu.memory[raw_operand as usize + 1],
                    cpu.memory[raw_operand as usize + 2], cpu.memory[raw_operand as usize + 3]
                ]);
                #[allow(clippy::cast_possible_truncation)]
                if target > (cpu.memory.len() - 4) as u32 {
                    Err(Interrupt::IllegalMemory)
                } else {
                    Self::memory(target, cpu)
                }
            }
            Addressing::Implied => Ok(Self::Void)
        }
    }

    fn direct(register: u32, cpu: &CPU) -> CPUResult<Self> {
        let register = RegisterId::try_from(register)?;
        let value = cpu.get_register(register);

        Ok(Operand::Register(register, value))
    }

    fn memory(address: u32, cpu: &CPU) -> CPUResult<Self> {
        #[allow(clippy::cast_possible_truncation)]
        if address > (cpu.memory.len() - 4) as u32 {
            return Err(Interrupt::IllegalMemory);
        }

        let value_word = u32::from_le_bytes([
            cpu.memory[address as usize],
            cpu.memory[address as usize + 1],
            cpu.memory[address as usize + 2],
            cpu.memory[address as usize + 3]
        ]);
        Ok(Operand::Memory(address, value_word))
    }

    #[must_use]
    pub fn disassemble(raw_operand: u32, cpu: &CPU, mode: Addressing) -> String {
        if let Ok(operand) = Operand::decode(raw_operand, cpu, mode) {
            operand.disassemble_self()
        } else {
            String::from("<unk>")
        }
    }

    #[must_use]
    pub fn disassemble_self(&self) -> String {
        match self {
            Self::Literal(value) => format!("#${value:X}"),
            Self::Register(id, _) => format!("{id:?}").to_lowercase(),
            Self::Memory(address, _) => format!("${address:0>8x}"),
            Self::Void => String::new(),
        }
    }
}
