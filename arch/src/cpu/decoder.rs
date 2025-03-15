use alloc::string::String;
use crate::core::binary::ExtractedBinaryData;
use crate::core::{Instruction, Interrupt};
use crate::core::instruction::DecodedInstruction;
use crate::CPU;
use crate::CPUResult;

pub trait Decoder {
    fn extract_instruction(&self, position: u32) -> CPUResult<ExtractedBinaryData>;
    fn extract_instruction_infailible(&self, position: u32) -> ExtractedBinaryData;
    fn decode_instruction(&self, position: u32) -> CPUResult<DecodedInstruction>;
    fn read_instruction(&self, position: u32) -> CPUResult<Instruction>;
    fn read_instruction_string(&self, position: u32) -> String;
}

impl Decoder for CPU {
    fn extract_instruction(&self, position: u32) -> CPUResult<ExtractedBinaryData> {
        let index = position as usize;
        if index + 15 < self.memory.len() {
            Ok(ExtractedBinaryData(&self.memory[index..index + 15]))
        } else {
            Err(Interrupt::IllegalMemory)
        }
    }

    fn extract_instruction_infailible(&self, position: u32) -> ExtractedBinaryData {
        let index = position as usize;
        if index + 15 < self.memory.len() {
            ExtractedBinaryData(&self.memory[index..index + 15])
        } else {
            ExtractedBinaryData(&[0u8; 0])
        }
    }

    fn decode_instruction(&self, position: u32) -> CPUResult<DecodedInstruction> {
        let opcode = self.extract_instruction(position)?.0;

        let instruction = u32::from_le_bytes([
            opcode[0], opcode[1],
            opcode[2], 0
        ]);

        let operands = [
            u32::from_le_bytes([
                opcode[3], opcode[4],
                opcode[5], opcode[6]
            ]),
            u32::from_le_bytes([
                opcode[7], opcode[8],
                opcode[9], opcode[10]
            ]),
            u32::from_le_bytes([
                opcode[11], opcode[12],
                opcode[13], opcode[14]
            ])
        ];

        let modes = [
            instruction & 0xF,
            (instruction & 0xF0) >> 4,
            (instruction & 0xF00) >> 8,
        ];

        let operation = instruction >> 12;

        Ok(DecodedInstruction {
            instruction,
            operands,
            modes,
            operation
        })
    }

    fn read_instruction(&self, position: u32) -> CPUResult<Instruction> {
        let instruction = self.decode_instruction(position)?;
        instruction.into_instruction(self)
    }

    fn read_instruction_string(&self, position: u32) -> String {
        if let Ok(instruction) = self.decode_instruction(position) {
            instruction.disassemble(self)
        } else {
            String::from("<invalid>")
        }
    }
}
