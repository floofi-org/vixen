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
        ExtractedBinaryData(&self.memory[index..index + 10])
    }

    fn read_instruction(&self, position: u32) -> CPUResult<Instruction> {
        let opcode = self.extract_instruction(position).0;

        let instruction = u16::from(opcode[9]) * 0x10 + u16::from(opcode[8] >> 4);
        let mode = Addressing::try_from(opcode[8] & 0x0F)?;
        let operation = Operation::try_from(instruction)?;

        let operand1 = Operand::decode(
            u32::from_le_bytes([
                opcode[4], opcode[5],
                opcode[6], opcode[7]
            ]),
            self, mode
        )?;
        let operand2 = Operand::decode(
            u32::from_le_bytes([
                opcode[0], opcode[1],
                opcode[2], opcode[3]
            ]),
            self, mode
        )?;

        Ok(Instruction {
            operation,
            mode,
            operands: [operand1, operand2],
        })
    }

    fn read_instruction_string(&self, position: u32, disassembler_mode: bool) -> String {
        let mut disassembled = String::new();

        let opcode = self.extract_instruction(position).0;
        let instruction = u16::from(opcode[9]) * 0x10 + u16::from(opcode[8] >> 4);
        let mode = opcode[4] & 0x0F;

        disassembled.push_str(&Operation::disassemble(instruction, mode, disassembler_mode));

        if let Ok(mode) = Addressing::try_from(opcode[8] & 0x0F) {
            if mode != Addressing::Implied {
                disassembled.push_str(&Operand::disassemble(
                    u32::from_le_bytes([
                        opcode[4], opcode[5],
                        opcode[6], opcode[7]
                    ]),
                    self, mode, disassembler_mode));
                if mode == Addressing::Immediate || u32::from_le_bytes([
                    opcode[0], opcode[1],
                    opcode[2], opcode[3]
                ]) != 0 {
                    disassembled.push_str(", ");
                    disassembled.push_str(&Operand::disassemble(
                        u32::from_le_bytes([
                            opcode[0], opcode[1],
                            opcode[2], opcode[3]
                        ]),
                        self, mode, disassembler_mode));
                }
            }
        } else {
            disassembled.push_str(if disassembler_mode {
                "<unk>"
            } else {
                "<invalid memory mode>"
            });
        }

        if disassembler_mode {
            disassembled = format!("{disassembled:<32}; {position:0>8x}: {}",
                                   self.extract_instruction(position));
        }

        disassembled
    }
}
