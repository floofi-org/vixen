use crate::core::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn int(_mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    let interrupt_code = cpu.registers.a & 0b00001111;

    Err(match interrupt_code {
        0x0 => Interrupt::User1,
        0x1 => Interrupt::User2,
        0x2 => Interrupt::User3,
        0x3 => Interrupt::User4,
        0x4 => Interrupt::User5,
        0x5 => Interrupt::User6,
        0x6 => Interrupt::User7,
        0x7 => Interrupt::User8,
        0x8 => Interrupt::User9,
        0x9 => Interrupt::User10,
        0xA => Interrupt::User11,
        0xB => Interrupt::User12,
        0xC => Interrupt::User13,
        0xD => Interrupt::User14,
        0xE => Interrupt::User15,
        _ => Interrupt::User16
    })
}