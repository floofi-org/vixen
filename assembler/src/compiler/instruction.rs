use vixen::core::instruction::Addressing;

use crate::models::{Instruction, Operand};

use super::{Compile, Compiler, CompilerError};

impl Compile for Instruction {
    fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        if self.operands.len() > 3 {
            return Err(CompilerError::TooManyOperands);
        }

        // Move by 3 nibbles to leave room for addressing modes
        let mut opcode = u32::from(u16::from(self.operation)) << 12;

        let operand_modes = self.operands.iter()
            .map(Operand::get_addressing);
        let empty_operands = 3 - operand_modes.len();

        let mut modes = [Addressing::Implied; 3];

        for (mode, operand_mode) in modes.iter_mut().zip(operand_modes) {
            *mode = operand_mode;
        }

        // Last 3 nibbles are addressing modes
        // See docs: https://github.com/floofi-org/vixen/wiki/Instruction-encoding
        //
        // Start from the first mode  and move to left by nibble each time
        for (i, mode) in modes.into_iter().enumerate() {
            let nibble = i * 4;
            let mode = u32::from(u8::from(mode)) & 0xF;

            opcode |= mode << nibble;
        }

        let opcode = opcode.to_le_bytes();

        // Opcode is 3 bytes long, so take 3 out of 4 in u32, skipping zero MSB
        for byte in opcode.into_iter().take(3) {
            compiler.write_byte(byte);
        }

        for operand in self.operands {
            operand.compile(compiler)?;
        }

        for _ in 0..empty_operands {
            compiler.write_word(0);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use vixen::core::{instruction::Operation, registers::RegisterId};

    use crate::{models::Operand, parser::Program};

    use super::*;

    #[test]
    fn test_instruction() {
        let instruction = Instruction {
            operation: Operation::Sqre,
            operands: vec![Operand::Register(RegisterId::R0), Operand::Literal(32)],
        };

        let mut program = Program::default();
        program.instructions.push_back(instruction);

        let result = Compiler::default()
            .compile(program.instructions)
            .unwrap();

        #[allow(clippy::identity_op)]
        let expected: &[u8] = &[
            // Addressing modes 2 | 1
            0x00 | 0x1,
            // Instruction | Addressing mode 3
            0x70 | 0x5,
            // Instruction group
            0x01,
            // Operand 1: RegisterId::R0
            1, 0, 0, 0,
            // Operand 2: Literal 32
            32, 0, 0, 0,
            // Operand 3: Void
            0, 0, 0, 0,
        ];


        assert_eq!(result, expected);
    }
}
