use alloc::string::String;
use crate::core::Operand;
use crate::{instructions, CPUResult, InstructionResult};
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
pub struct DecodedInstruction {
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
            Operation::Sqrt => instructions::sqrt(&mut self.operands, cpu),
            Operation::Cbrt => instructions::cbrt(&mut self.operands, cpu),
            Operation::Sqre => instructions::sqre(&mut self.operands, cpu),
            Operation::Cube => instructions::cube(&mut self.operands, cpu),
            Operation::Min => instructions::min(&mut self.operands, cpu),
            Operation::Max => instructions::max(&mut self.operands, cpu),
            Operation::Addc => instructions::addc(&mut self.operands, cpu),
            Operation::Subc => instructions::subc(&mut self.operands, cpu),
            Operation::Sar => instructions::sar(&mut self.operands, cpu),
            Operation::Sal => instructions::sal(&mut self.operands, cpu),

            // 0x02?? - Logic Instructions
            Operation::And => instructions::and(&mut self.operands, cpu),
            Operation::Or => instructions::or(&mut self.operands, cpu),
            Operation::Xor => instructions::xor(&mut self.operands, cpu),
            Operation::Nor => instructions::nor(&mut self.operands, cpu),
            Operation::Nand => instructions::nand(&mut self.operands, cpu),
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
            Operation::Cmp => instructions::cmp(&mut self.operands, cpu),
            Operation::Lte => instructions::lte(&mut self.operands, cpu),
            Operation::Gte => instructions::gte(&mut self.operands, cpu),
            Operation::Setz => instructions::setz(&mut self.operands, cpu),
            Operation::Setc => instructions::setc(&mut self.operands, cpu),
            Operation::Seto => instructions::seto(&mut self.operands, cpu),

            // 0x05?? - Data Movement Instructions
            Operation::Mov => instructions::mov(&mut self.operands, cpu),
            Operation::Xchg => instructions::xchg(&mut self.operands, cpu),
            Operation::Clr => instructions::clr(&mut self.operands, cpu),
            Operation::Stc => instructions::stc(&self.operands, cpu),
            Operation::Clc => instructions::clc(&self.operands, cpu),
            Operation::Sti => instructions::sti(&self.operands, cpu),
            Operation::Cli => instructions::cli(&self.operands, cpu),
            Operation::Clv => instructions::clv(&self.operands, cpu),

            // 0x06?? - Control Flow Instructions
            Operation::Jmp => instructions::jmp(&mut self.operands, cpu),
            Operation::Jmpl => instructions::jmpl(&mut self.operands, cpu),
            Operation::Ret => instructions::ret(&self.operands, cpu),
            Operation::Jz => instructions::jz(&mut self.operands, cpu),
            Operation::Jnz => instructions::jnz(&mut self.operands, cpu),
            Operation::Jc => instructions::jc(&mut self.operands, cpu),
            Operation::Jnc => instructions::jnc(&mut self.operands, cpu),
            Operation::Jo => instructions::jo(&mut self.operands, cpu),
            Operation::Jno => instructions::jno(&mut self.operands, cpu),
            Operation::Int => instructions::int(&self.operands, cpu),
            Operation::Iret => instructions::iret(&self.operands, cpu),
            Operation::Irets => instructions::irets(&self.operands, cpu),
            Operation::Nop => instructions::nop(&self.operands, cpu),
            Operation::Hlt => instructions::hlt(&self.operands, cpu),
            Operation::Js => instructions::js(&mut self.operands, cpu),
            Operation::Jns => instructions::jns(&mut self.operands, cpu),

            // 0x07?? - Stack Instructions
            Operation::Push => instructions::push(&mut self.operands, cpu),
            Operation::Pop => instructions::pop(&mut self.operands, cpu),
            Operation::Pushf => instructions::pushf(&self.operands, cpu),
            Operation::Popf => instructions::popf(&self.operands, cpu)
        }
    }
}

impl DecodedInstruction {
    #[allow(clippy::cast_possible_truncation)]
    pub fn into_instruction(self, cpu: &CPU) -> CPUResult<Instruction> {
        let operation = Operation::try_from(self.operation as u16)?;
        let modes = [
            Addressing::try_from(self.modes[0] as u8)?,
            Addressing::try_from(self.modes[1] as u8)?,
            Addressing::try_from(self.modes[2] as u8)?
        ];
        let operands = [
            Operand::decode(self.operands[0], cpu, modes[0])?,
            Operand::decode(self.operands[1], cpu, modes[1])?,
            Operand::decode(self.operands[2], cpu, modes[2])?
        ];
        Ok(Instruction {
            operation,
            operands,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use] pub fn disassemble(self, cpu: &CPU) -> String {
        let mut disassembled = String::new();
        let operation = Operation::disassemble(self.operation as u16);
        disassembled.push_str(&operation);

        let operands_with_modes = [
            (self.operands[0], self.modes[0]),
            (self.operands[1], self.modes[1]),
            (self.operands[2], self.modes[2])
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

                let operand = Operand::disassemble(*operand, cpu, mode);
                disassembled.push_str(&operand);
            }
        }

        disassembled
    }
}
