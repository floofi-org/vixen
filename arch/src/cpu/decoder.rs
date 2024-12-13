use alloc::format;
use alloc::string::String;
use crate::core::binary::ExtractedBinaryData;
use crate::core::Instruction;
use crate::core::instruction::Addressing;
use crate::core::instruction::Operation;
use crate::core::Operand;
use crate::CPU;
use crate::CPUResult;

pub trait Decoder {
    fn extract_instruction(&self, position: u32) -> ExtractedBinaryData;
    fn read_instruction(&self, position: u32) -> CPUResult<Instruction>;
    fn read_instruction_string(&self, position: u32, disassembler_mode: bool) -> String;
}

impl Decoder for CPU {
    fn extract_instruction(&self, position: u32) -> ExtractedBinaryData {
        let index = position as usize;
        ExtractedBinaryData(&self.memory[index..index + 15])
    }

    fn read_instruction(&self, position: u32) -> CPUResult<Instruction> {
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

        println!("{modes:x?}");

        let operation = instruction >> 12;
        #[allow(clippy::cast_possible_truncation)]
        let operation = Operation::try_from(operation as u16)?;

        let modes = [
            Addressing::try_from(modes[0] as u8)?,
            Addressing::try_from(modes[1] as u8)?,
            Addressing::try_from(modes[2] as u8)?
        ];

        let operands = [
            Operand::decode(operands[0], self, modes[0])?,
            Operand::decode(operands[1], self, modes[1])?,
            Operand::decode(operands[2], self, modes[2])?
        ];

        Ok(Instruction {
            operation,
            modes,
            operands,
        })
    }

    fn read_instruction_string(&self, position: u32, disassembler_mode: bool) -> String {
        let mut disassembled = String::new();

        let opcode = self.extract_instruction(position).0;

        let instruction = u32::from_le_bytes([
            opcode[0], opcode[1],
            opcode[2], 0
        ]);

        let operands_with_modes = [
            (
                u32::from_le_bytes([
                    opcode[3], opcode[4],
                    opcode[5], opcode[6]
                ]),
                instruction & 0xF
            ),
            (
                u32::from_le_bytes([
                    opcode[7], opcode[8],
                    opcode[9], opcode[10]
                ]),
                (instruction & 0xF0) >> 4
            ),
            (
                u32::from_le_bytes([
                    opcode[11], opcode[12],
                    opcode[13], opcode[14]
                ]),
                (instruction & 0xF00) >> 8
            )
        ];

        let operation = instruction >> 12;
        #[allow(clippy::cast_possible_truncation)]
        disassembled.push_str(&Operation::disassemble(operation as u16, operation, disassembler_mode));

        for (i, (operand, mode)) in operands_with_modes.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            if let Ok(mode) = Addressing::try_from(*mode as u8) {
                if mode != Addressing::Implied {
                    if i > 0 {
                        disassembled.push_str(", ");
                    }
                    disassembled.push_str(&Operand::disassemble(*operand, self, mode, disassembler_mode));
                }
            } else {
                disassembled.push_str(if disassembler_mode {
                    "<unk>"
                } else {
                    "<invalid memory mode>"
                });
            }
        }

        if disassembler_mode {
            disassembled = format!("{disassembled:<48}; {position:0>8x}: {}",
                                   self.extract_instruction(position));
        }

        disassembled
    }
}
