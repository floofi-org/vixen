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
            Operation::Add => instructions::add(&mut self.operands, cpu),
            Operation::Sub => instructions::sub(&mut self.operands, cpu),
            Operation::Mul => instructions::mul(&mut self.operands, cpu),
            Operation::Div => instructions::div(&mut self.operands, cpu),
            Operation::Mod => instructions::r#mod(&mut self.operands, cpu),
            Operation::Sqt => instructions::sqt(&mut self.operands, cpu),
            Operation::Cbt => instructions::cbt(&mut self.operands, cpu),
            Operation::Sqr => instructions::sqr(&mut self.operands, cpu),
            Operation::Cbe => instructions::cbe(&mut self.operands, cpu),
            Operation::Min => instructions::min(&mut self.operands, cpu),
            Operation::Max => instructions::max(&mut self.operands, cpu),
            Operation::Adc => instructions::adc(&mut self.operands, cpu),
            Operation::Sbc => instructions::sbc(&mut self.operands, cpu),
            Operation::Asr => instructions::asr(&mut self.operands, cpu),

            // 0x02?? - Logic Instructions
            Operation::And => instructions::and(&mut self.operands, cpu),
            Operation::Or => instructions::or(&mut self.operands, cpu),
            Operation::Xor => instructions::xor(&mut self.operands, cpu),
            Operation::Nor => instructions::nor(&mut self.operands, cpu),
            Operation::Nad => instructions::nad(&mut self.operands, cpu),
            Operation::Imp => instructions::imp(&mut self.operands, cpu),
            Operation::Not => instructions::not(&self.operands, cpu),
            Operation::Shl => instructions::shl(&mut self.operands, cpu),
            Operation::Shr => instructions::shr(&mut self.operands, cpu),
            Operation::Rol => instructions::rol(&mut self.operands, cpu),
            Operation::Ror => instructions::ror(&mut self.operands, cpu),

            // 0x03?? - Counting Instructions
            Operation::Inc => instructions::inc(&mut self.operands, cpu),
            Operation::Dec => instructions::dec(&mut self.operands, cpu),

            // 0x04?? - Comparison Instructions
            Operation::Cmp => instructions::cmp(&self.operands, cpu),
            Operation::Lte => instructions::lte(&self.operands, cpu),
            Operation::Gte => instructions::gte(&self.operands, cpu),
            Operation::Srz => instructions::srz(&mut self.operands, cpu),
            Operation::Src => instructions::src(&mut self.operands, cpu),
            Operation::Sro => instructions::sro(&mut self.operands, cpu),

            // 0x05?? - Data Movement Instructions
            Operation::Mov => instructions::mov(&mut self.operands, cpu),
            Operation::Swp => instructions::swp(&mut self.operands, cpu),
            Operation::Clr => instructions::clr(&mut self.operands, cpu),
            Operation::Sec => instructions::sec(&self.operands, cpu),
            Operation::Clc => instructions::clc(&self.operands, cpu),
            Operation::Sei => instructions::sei(&self.operands, cpu),
            Operation::Cli => instructions::cli(&self.operands, cpu),
            Operation::Clv => instructions::clv(&self.operands, cpu),

            // 0x06?? - Control Flow Instructions
            Operation::Jmp => instructions::jmp(&self.operands, cpu),
            Operation::Jsr => instructions::jsr(&self.operands, cpu),
            Operation::Ret => instructions::ret(&self.operands, cpu),
            Operation::Beq => instructions::beq(&self.operands, cpu),
            Operation::Bne => instructions::bne(&self.operands, cpu),
            Operation::Bec => instructions::bec(&self.operands, cpu),
            Operation::Bnc => instructions::bnc(&self.operands, cpu),
            Operation::Beo => instructions::beo(&self.operands, cpu),
            Operation::Bno => instructions::bno(&self.operands, cpu),
            Operation::Int => instructions::int(&self.operands, cpu),
            Operation::Irt => instructions::irt(&self.operands, cpu),
            Operation::Nop => instructions::nop(&self.operands, cpu),
            Operation::Jam => instructions::jam(&self.operands, cpu),
            Operation::Bpl => instructions::bpl(&self.operands, cpu),
            Operation::Bmi => instructions::bmi(&self.operands, cpu),

            // 0x07?? - Stack Instructions
            Operation::Psh => instructions::psh(&self.operands, cpu),
            Operation::Pll => instructions::pll(&mut self.operands, cpu),
            Operation::Php => instructions::php(&self.operands, cpu),
            Operation::Plp => instructions::plp(&self.operands, cpu)
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
            let mode = Addressing::try_from(*mode as u8)
                .ok()
                .filter(|m| *m != Addressing::Implied);

            if let Some(mode) = mode {
                if i > 0 {
                    disassembled.push_str(", ");
                }
                
                let operand = Operand::disassemble(*operand, value.cpu, mode);
                disassembled.push_str(&operand);         
            }
        }

        disassembled
    }
}
