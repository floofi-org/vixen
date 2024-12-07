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
    ZeroPage(u32, u32),
    Memory(u32, u32),
    Void
}

impl Operand {
    pub fn decode(raw_operand: u32, cpu: &CPU, mode: Addressing) -> CPUResult<Operand> {
        match &mode {
            Addressing::Immediate => Ok(Self::Literal(raw_operand)),
            Addressing::Direct => Self::direct(raw_operand, cpu),
            Addressing::ZeroPage => Self::zero_page(raw_operand, cpu),
            Addressing::Absolute => Self::memory(raw_operand, cpu),
            Addressing::Relative => {
                // Relative offsets are stored as u16 internally
                // but need to be interpreted as i16.
                #[allow(clippy::cast_possible_wrap)]
                let offset = raw_operand as i32;
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
            Addressing::Implied => Ok(Self::Void)
        }
    }

    fn direct(register: u32, cpu: &CPU) -> CPUResult<Self> {
        let register = RegisterId::try_from(register)?;
        let value = cpu.get_register(register);

        Ok(Operand::Register(register, value))
    }

    fn zero_page(address: u32, cpu: &CPU) -> CPUResult<Self> {
        if address > 0xFF {
            return Err(Interrupt::IllegalMemory);
        }

        let value_word = u32::from_le_bytes([
            cpu.memory[address as usize],
            cpu.memory[address as usize + 1],
            cpu.memory[address as usize + 2],
            cpu.memory[address as usize + 3]
        ]);
        Ok(Operand::ZeroPage(address, value_word))
    }

    fn memory(address: u32, cpu: &CPU) -> CPUResult<Self> {
        if address <= 0xFF {
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
    pub fn disassemble(raw_operand: u32, cpu: &CPU, mode: Addressing, disassembler_mode: bool) -> String {
        if let Ok(operand) = Operand::decode(raw_operand, cpu, mode) {
            operand.disassemble_self()
        } else if disassembler_mode {
            String::from("<unk>")
        } else {
            format!("??({raw_operand:0>8x})")
        }
    }

    #[must_use]
    pub fn disassemble_self(&self) -> String {
        match self {
            Self::Literal(value) => format!("#${value:X}"),
            Self::Register(id, _) => format!("{id:?}"),
            Self::ZeroPage(address, _) => format!("${address:0>8X}"),
            Self::Memory(address, _) => format!("${address:0>8x}"),
            Self::Void => String::new(),
        }
    }
}
