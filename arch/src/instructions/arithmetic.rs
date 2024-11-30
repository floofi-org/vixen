use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;
use libm::{sqrtf, cbrtf};

pub fn add(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn sub(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn mul(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn div(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn mod_(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn sqt(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number = operands[0].read_word()?;
        let result = sqrtf(number as f32) as u8;
        let result_negative = result >> 7 == 1;

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cbt(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number = operands[0].read_word()?;
        let result = cbrtf(number as f32) as u8;
        let result_negative = result >> 7 == 1;

        cpu.status_register.zero = result == 0;
        cpu.status_register.negative = result_negative;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sqr(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn cbe(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn min(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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

pub fn max(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
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