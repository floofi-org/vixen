use alloc::string::String;
use crate::core::binary::ExtractedBinaryData;
use crate::core::Instruction;
use crate::core::instruction::Addressing;
use crate::core::instruction::Operation;
use crate::core::Operand;
use crate::CPU;
use crate::CPUResult;

pub trait Decoder {
    fn extract_instruction(&self, position: u16) -> ExtractedBinaryData;
    fn read_instruction(&self, position: u16) -> CPUResult<Instruction>;
    fn read_instruction_string(&self, position: u16) -> String;
}

impl Decoder for CPU {
    fn extract_instruction(&self, position: u16) -> ExtractedBinaryData {
        let index = position as usize;
        ExtractedBinaryData(&self.memory[index..index + 6])
    }

    fn read_instruction(&self, position: u16) -> CPUResult<Instruction> {
        let opcode = self.extract_instruction(position).0;

        let instruction = u16::from(opcode[5]) * 0x10 + u16::from(opcode[4] >> 4);
        let mode = Addressing::try_from(opcode[4] & 0x0F)?;
        let operation = Operation::try_from(instruction)?;

        let operand1 = Operand::decode(
            u16::from(opcode[3]) * 0x100 + u16::from(opcode[2]),
            self, mode
        )?;
        let operand2 = Operand::decode(
            u16::from(opcode[1]) * 0x100 + u16::from(opcode[0]),
            self, mode
        )?;

        Ok(Instruction {
            operation,
            mode,
            operands: [operand1, operand2],
        })
    }

    fn read_instruction_string(&self, position: u16) -> String {
        let mut disassembled = String::new();

        let opcode = self.extract_instruction(position).0;
        let instruction = u16::from(opcode[5]) * 0x10 + u16::from(opcode[4] >> 4);
        let mode = opcode[4] & 0x0F;

        disassembled.push_str(&Operation::disassemble(instruction, mode));

        if let Ok(mode) = Addressing::try_from(opcode[4] & 0x0F) {
            if mode != Addressing::Implied {
                disassembled.push_str(&Operand::disassemble(
                    u16::from(opcode[3]) * 0x100 + u16::from(opcode[2]),
                    self, mode));
                if mode == Addressing::Immediate || u16::from(opcode[1]) * 0x100 + u16::from(opcode[0]) != 0 {
                    disassembled.push_str(", ");
                    disassembled.push_str(&Operand::disassemble(
                        u16::from(opcode[1]) * 0x100 + u16::from(opcode[0]),
                        self, mode));
                }
            }
        } else {
            disassembled.push_str("<invalid memory mode>");
        }

        disassembled
    }
}
