use alloc::string::String;
use crate::core::binary::ExtractedBinaryData;
use crate::core::Instruction;
use crate::core::instruction::DecodedInstruction;
use crate::CPU;
use crate::CPUResult;

pub trait Decoder {
    fn extract_instruction(&self, position: u32) -> ExtractedBinaryData;
    fn decode_instruction(&self, position: u32) -> DecodedInstruction;
    fn read_instruction(&self, position: u32) -> CPUResult<Instruction>;
    fn read_instruction_string(&self, position: u32) -> String;
}

impl Decoder for CPU {
    fn extract_instruction(&self, position: u32) -> ExtractedBinaryData {
        let index = position as usize;
        ExtractedBinaryData(&self.memory[index..index + 15])
    }

    fn decode_instruction(&self, position: u32) -> DecodedInstruction {
        let opcode = self.extract_instruction(position).0;

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

        DecodedInstruction {
            cpu: self,
            instruction,
            operands,
            modes,
            operation
        }
    }

    fn read_instruction(&self, position: u32) -> CPUResult<Instruction> {
        let instruction = self.decode_instruction(position);
        instruction.try_into()
    }

    fn read_instruction_string(&self, position: u32) -> String {
        let instruction = self.decode_instruction(position);
        instruction.into()
    }
}