use crate::core::binary::ExtractedBinaryData;
use crate::core::instruction::Instruction;
use crate::core::instruction_mode::InstructionMode;
use crate::core::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::CPUResult;

pub trait CPUDecoder {
    fn extract_instruction(&self, position: u16) -> ExtractedBinaryData;
    fn read_instruction(&self, position: u16) -> CPUResult<Instruction>;
}

impl CPUDecoder for CPU {
    fn extract_instruction(&self, position: u16) -> ExtractedBinaryData {
        let index = position as usize;
        ExtractedBinaryData(&self.memory[index..index + 6])
    }

    fn read_instruction(&self, position: u16) -> CPUResult<Instruction> {
        let opcode = self.extract_instruction(position).0;

        let instruction = opcode[5] as u16 * 0x10 + (opcode[4] >> 4) as u16;
        let mode = InstructionMode::try_from(opcode[4] & 0x0F)?;
        let operation = InstructionOperation::try_from(instruction)?;

        let operand1 = Operand::decode(
            opcode[3] as u16 * 0x100 + opcode[2] as u16,
            self, mode
        )?;
        let operand2 = Operand::decode(
            opcode[1] as u16 * 0x100 + opcode[0] as u16,
            self, mode
        )?;

        Ok(Instruction {
            operation,
            mode,
            operands: [operand1, operand2],
        })
    }
}