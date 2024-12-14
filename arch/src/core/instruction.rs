use alloc::string::String;
use crate::core::{Interrupt, Operand};
use crate::{instructions, InstructionResult};
use crate::CPU;

pub mod addressing;
pub mod operation;

pub use addressing::Addressing;
pub use operation::Operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub modes: [Addressing; 3],
    pub operands: [Operand; 3]
}

#[allow(clippy::module_name_repetitions)]
pub struct DecodedInstruction<'a> {
    pub cpu: &'a CPU,
    pub instruction: u32,
    pub operands: [u32; 3],
    pub modes: [u32; 3],
    pub operation: u32
}

impl Instruction {
    pub fn execute_unhandled(&mut self, cpu: &mut CPU) -> InstructionResult {
        match self.operation {
            // 0x01?? - Arithmetic and Algebric Instructions
            Operation::Add => instructions::add(&self.modes, &self.operands, cpu),
            Operation::Sub => instructions::sub(&self.modes, &self.operands, cpu),
            Operation::Mul => instructions::mul(&self.modes, &self.operands, cpu),
            Operation::Div => instructions::div(&self.modes, &self.operands, cpu),
            Operation::Mod => instructions::mod_(&self.modes, &self.operands, cpu),
            Operation::Sqt => instructions::sqt(&self.modes, &self.operands, cpu),
            Operation::Cbt => instructions::cbt(&self.modes, &self.operands, cpu),
            Operation::Sqr => instructions::sqr(&self.modes, &self.operands, cpu),
            Operation::Cbe => instructions::cbe(&self.modes, &self.operands, cpu),
            Operation::Min => instructions::min(&self.modes, &self.operands, cpu),
            Operation::Max => instructions::max(&self.modes, &self.operands, cpu),
            Operation::Adc => instructions::adc(&self.modes, &self.operands, cpu),
            Operation::Sbc => instructions::sbc(&self.modes, &self.operands, cpu),
            Operation::Asr => instructions::asr(&self.modes, &mut self.operands, cpu),

            // 0x02?? - Logic Instructions
            Operation::And => instructions::and(&self.modes, &self.operands, cpu),
            Operation::Or => instructions::or(&self.modes, &self.operands, cpu),
            Operation::Xor => instructions::xor(&self.modes, &self.operands, cpu),
            Operation::Nor => instructions::nor(&self.modes, &self.operands, cpu),
            Operation::Nad => instructions::nad(&self.modes, &self.operands, cpu),
            Operation::Imp => instructions::imp(&self.modes, &self.operands, cpu),
            Operation::Not => instructions::not(&self.modes, &self.operands, cpu),
            Operation::Shl => instructions::shl(&self.modes, &mut self.operands, cpu),
            Operation::Shr => instructions::shr(&self.modes, &mut self.operands, cpu),
            Operation::Rol => instructions::rol(&self.modes, &mut self.operands, cpu),
            Operation::Ror => instructions::ror(&self.modes, &mut self.operands, cpu),

            // 0x03?? - Counting Instructions
            Operation::Inc => instructions::inc(&self.modes, &mut self.operands, cpu),
            Operation::Dec => instructions::dec(&self.modes, &mut self.operands, cpu),

            // 0x04?? - Comparison Instructions
            Operation::Cmp => instructions::cmp(&self.modes, &self.operands, cpu),
            Operation::Lte => instructions::lte(&self.modes, &self.operands, cpu),
            Operation::Gte => instructions::gte(&self.modes, &self.operands, cpu),
            Operation::Srz => instructions::srz(&self.modes, &mut self.operands, cpu),
            Operation::Src => instructions::src(&self.modes, &mut self.operands, cpu),
            Operation::Sro => instructions::sro(&self.modes, &mut self.operands, cpu),

            // 0x05?? - Data Movement Instructions
            Operation::Ldr => instructions::ldr(&self.modes, &mut self.operands, cpu),
            Operation::Str => instructions::str(&self.modes, &mut self.operands, cpu),
            Operation::Mov => instructions::mov(&self.modes, &mut self.operands, cpu),
            Operation::Swp => instructions::swp(&self.modes, &mut self.operands, cpu),
            Operation::Clr => instructions::clr(&self.modes, &mut self.operands, cpu),
            Operation::Sec => instructions::sec(&self.modes, &self.operands, cpu),
            Operation::Clc => instructions::clc(&self.modes, &self.operands, cpu),
            Operation::Sei => instructions::sei(&self.modes, &self.operands, cpu),
            Operation::Cli => instructions::cli(&self.modes, &self.operands, cpu),
            Operation::Clv => instructions::clv(&self.modes, &self.operands, cpu),

            // 0x06?? - Control Flow Instructions
            Operation::Jmp => instructions::jmp(&self.modes, &self.operands, cpu),
            Operation::Jsr => instructions::jsr(&self.modes, &self.operands, cpu),
            Operation::Ret => instructions::ret(&self.modes, &self.operands, cpu),
            Operation::Beq => instructions::beq(&self.modes, &self.operands, cpu),
            Operation::Bne => instructions::bne(&self.modes, &self.operands, cpu),
            Operation::Bec => instructions::bec(&self.modes, &self.operands, cpu),
            Operation::Bnc => instructions::bnc(&self.modes, &self.operands, cpu),
            Operation::Beo => instructions::beo(&self.modes, &self.operands, cpu),
            Operation::Bno => instructions::bno(&self.modes, &self.operands, cpu),
            Operation::Int => instructions::int(&self.modes, &self.operands, cpu),
            Operation::Irt => instructions::irt(&self.modes, &self.operands, cpu),
            Operation::Nop => instructions::nop(&self.modes, &self.operands, cpu),
            Operation::Jam => instructions::jam(&self.modes, &self.operands, cpu),
            Operation::Bpl => instructions::bpl(&self.modes, &self.operands, cpu),
            Operation::Bmi => instructions::bmi(&self.modes, &self.operands, cpu),

            // 0x07?? - Stack Instructions
            Operation::Psh => instructions::psh(&self.modes, &self.operands, cpu),
            Operation::Pll => instructions::pll(&self.modes, &mut self.operands, cpu),
            Operation::Php => instructions::php(&self.modes, &self.operands, cpu),
            Operation::Plp => instructions::plp(&self.modes, &self.operands, cpu)
        }
    }
}

impl TryFrom<DecodedInstruction<'_>> for Instruction {
    type Error = Interrupt;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: DecodedInstruction) -> Result<Self, Self::Error> {
        let operation = Operation::try_from(value.operation as u16)?;
        let modes = [
            Addressing::try_from(value.modes[0] as u8)?,
            Addressing::try_from(value.modes[1] as u8)?,
            Addressing::try_from(value.modes[2] as u8)?
        ];
        let operands = [
            Operand::decode(value.operands[0], value.cpu, modes[0])?,
            Operand::decode(value.operands[1], value.cpu, modes[1])?,
            Operand::decode(value.operands[2], value.cpu, modes[2])?
        ];
        Ok(Instruction {
            operation,
            modes,
            operands,
        })
    }
}

impl From<DecodedInstruction<'_>> for String {
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: DecodedInstruction) -> Self {
        let mut disassembled = Self::new();
        let operation = Operation::disassemble(value.operation as u16);
        disassembled.push_str(&operation);

        let operands_with_modes = [
            (value.operands[0], value.modes[0]),
            (value.operands[1], value.modes[1]),
            (value.operands[2], value.modes[2])
        ];

        for (i, (operand, mode)) in operands_with_modes.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            if let Ok(mode) = Addressing::try_from(*mode as u8) {
                if mode != Addressing::Implied {
                    if i > 0 {
                        disassembled.push_str(", ");
                    }
                    disassembled.push_str(&Operand::disassemble(*operand, value.cpu, mode));
                }
            } else {
                disassembled.push_str("<unk>");
            }
        }

        disassembled
    }
}
