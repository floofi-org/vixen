use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;
use libm::{sqrtf, cbrtf};

pub fn add(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number1_negative = number1 >> 7 == 1;
        let number2 = operands[1].read_word()?;
        let number2_negative = number2 >> 7 == 1;

        let sum = number1.overflowing_add(number2);
        let sum_negative = sum.0 >> 7 == 1;

        cpu.status_register.carry = sum.1;
        cpu.status_register.overflow = (number1_negative == number2_negative) && (sum_negative != number1_negative);
        cpu.status_register.zero = sum.0 == 0;
        cpu.status_register.negative = sum_negative;
        cpu.registers.a = sum.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sub(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number1_negative = number1 >> 7 == 1;
        let number2 = operands[1].read_word()?;
        let number2_negative = number2 >> 7 == 1;

        let diff = number1.overflowing_sub(number2);
        let diff_negative = diff.0 >> 7 == 1;

        cpu.status_register.carry = diff.1;
        cpu.status_register.overflow = (number1_negative != number2_negative) && (diff_negative != number1_negative);
        cpu.status_register.zero = diff.0 == 0;
        cpu.status_register.negative = diff_negative;
        cpu.registers.a = diff.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn mul(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number1_negative = number1 >> 7 == 1;
        let number2 = operands[1].read_word()?;
        let number2_negative = number2 >> 7 == 1;

        let result = number1.overflowing_mul(number2);
        let result_negative = result.0 >> 7 == 1;

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = ((number1_negative == number2_negative) && result_negative) ||
            ((number1_negative != number2_negative) && !result_negative);
        cpu.status_register.zero = result.0 == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn div(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;

        if number2 == 0 { return Err(Interrupt::DivideByZero); }

        let result = number1.overflowing_div(number2);
        let result_negative = result.0 >> 7 == 1;

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = number1 == 128 && number2 == 255;
        cpu.status_register.zero = result.0 == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn mod_(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;

        if number2 == 0 { return Err(Interrupt::DivideByZero); }

        let result = number1 % number2;
        let result_negative = result >> 7 == 1;

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
// Square root should be unsigned and 8-bit, this is intended
pub fn sqt(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let result = sqrtf(f32::from(number)) as u8;
        let result_negative = result >> 7 == 1;

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
// Cube root should be unsigned and 8-bit, this is intended
pub fn cbt(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let result = cbrtf(f32::from(number)) as u8;
        let result_negative = result >> 7 == 1;

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

#[allow(clippy::cast_possible_wrap)] // Overflow is (by definition) for signed operations
pub fn sqr(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let number_abs = (number as i8).unsigned_abs();

        let result = number.overflowing_pow(2);
        let result_negative = result.0 >> 7 == 1;

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = number_abs > 11;
        cpu.status_register.zero = result.0 == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

#[allow(clippy::cast_possible_wrap)] // Overflow is (by definition) for signed operations
pub fn cbe(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let number_abs = (number as i8).unsigned_abs();

        let result = number.overflowing_pow(3);
        let result_negative = result.0 >> 7 == 1;

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = number_abs > 5;
        cpu.status_register.zero = result.0 == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn min(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operand[0].read_word()?;
        let number2 = operand[1].read_word()?;

        let result = number1.min(number2);

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result >> 7 == 1;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn max(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operand[0].read_word()?;
        let number2 = operand[1].read_word()?;

        let result = number1.max(number2);

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result >> 7 == 1;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
