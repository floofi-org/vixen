use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;
use libm::{sqrt, cbrt};

pub fn add(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number1_negative = number1 >> 31 == 1;
    let number2 = operands[1].read_word()?;
    let number2_negative = number2 >> 31 == 1;

    let sum = number1.overflowing_add(number2);
    let sum_negative = sum.0 >> 31 == 1;

    cpu.status_register.carry = sum.1;
    cpu.status_register.overflow = (number1_negative == number2_negative) && (sum_negative != number1_negative);
    cpu.status_register.zero = sum.0 == 0;
    cpu.status_register.negative = sum_negative;
    cpu.registers.r0 = sum.0;

    Ok(())
}

pub fn sub(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number1_negative = number1 >> 31 == 1;
    let number2 = operands[1].read_word()?;
    let number2_negative = number2 >> 31 == 1;

    let diff = number1.overflowing_sub(number2);
    let diff_negative = diff.0 >> 31 == 1;

    cpu.status_register.carry = diff.1;
    cpu.status_register.overflow = (number1_negative != number2_negative) && (diff_negative != number1_negative);
    cpu.status_register.zero = diff.0 == 0;
    cpu.status_register.negative = diff_negative;
    cpu.registers.r0 = diff.0;

    Ok(())
}

pub fn mul(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number1_negative = number1 >> 31 == 1;
    let number2 = operands[1].read_word()?;
    let number2_negative = number2 >> 31 == 1;

    let result = number1.overflowing_mul(number2);
    let result_negative = result.0 >> 31 == 1;

    cpu.status_register.carry = result.1;
    cpu.status_register.overflow = ((number1_negative == number2_negative) && result_negative) ||
        ((number1_negative != number2_negative) && !result_negative);
    cpu.status_register.zero = result.0 == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result.0;

    Ok(())
}

pub fn div(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number2 = operands[1].read_word()?;

    if number2 == 0 { return Err(Interrupt::DivideByZero); }

    let result = number1.overflowing_div(number2);
    let result_negative = result.0 >> 31 == 1;

    cpu.status_register.carry = result.1;
    cpu.status_register.overflow = number1 == 2_147_483_647 && number2 == 4_294_967_295;
    cpu.status_register.zero = result.0 == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result.0;

    Ok(())
}

pub fn mod_(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number2 = operands[1].read_word()?;

    if number2 == 0 { return Err(Interrupt::DivideByZero); }

    let result = number1 % number2;
    let result_negative = result >> 31 == 1;

    cpu.status_register.zero = result == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result;

    Ok(())
}

// Square root should be unsigned and 8-bit, this is intended
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn sqt(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word()?;
    let result = sqrt(f64::from(number)) as u32;
    let result_negative = result >> 31 == 1;

    cpu.status_register.zero = result == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result;

    Ok(())
}

// Cube root should be unsigned and 8-bit, this is intended
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn cbt(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word()?;
    let result = cbrt(f64::from(number)) as u32;
    let result_negative = result >> 31 == 1;

    cpu.status_register.zero = result == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result;

    Ok(())
}

// Overflow is (by definition) for signed operations
#[allow(clippy::cast_possible_wrap)]
pub fn sqr(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word()?;
    let number_abs = (number as i32).unsigned_abs();

    let result = number.overflowing_pow(2);
    let result_negative = result.0 >> 31 == 1;

    cpu.status_register.carry = result.1;
    cpu.status_register.overflow = number_abs > 65535;
    cpu.status_register.zero = result.0 == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result.0;

    Ok(())
}

// Overflow is (by definition) for signed operations
#[allow(clippy::cast_possible_wrap)]
pub fn cbe(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word()?;
    let number_abs = (number as i32).unsigned_abs();

    let result = number.overflowing_pow(3);
    let result_negative = result.0 >> 31 == 1;

    cpu.status_register.carry = result.1;
    cpu.status_register.overflow = number_abs > 1625;
    cpu.status_register.zero = result.0 == 0;
    cpu.status_register.negative = result_negative;
    cpu.registers.r0 = result.0;

    Ok(())
}

pub fn min(operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operand[0].read_word()?;
    let number2 = operand[1].read_word()?;

    let result = number1.min(number2);

    cpu.status_register.zero = result == 0;
    cpu.status_register.negative = result >> 31 == 1;
    cpu.registers.r0 = result;

    Ok(())
}

pub fn max(operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operand[0].read_word()?;
    let number2 = operand[1].read_word()?;

    let result = number1.max(number2);

    cpu.status_register.zero = result == 0;
    cpu.status_register.negative = result >> 31 == 1;
    cpu.registers.r0 = result;

    Ok(())
}

pub fn adc(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number1_negative = number1 >> 31 == 1;
    let number2 = operands[1].read_word()?;
    let number2_negative = number2 >> 31 == 1;

    let sum_pre = number1.overflowing_add(number2);
    let sum = sum_pre.0.overflowing_add(u32::from(cpu.status_register.carry));
    let sum_negative = sum_pre.0 >> 31 == 1;

    cpu.status_register.carry = sum.1;
    cpu.status_register.overflow = (number1_negative == number2_negative) && (sum_negative != number1_negative);
    cpu.status_register.zero = sum.0 == 0;
    cpu.status_register.negative = sum_negative;
    cpu.registers.r0 = sum.0;

    Ok(())
}

pub fn sbc(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[0].read_word()?;
    let number1_negative = number1 >> 31 == 1;
    let number2 = operands[1].read_word()?;
    let number2_negative = number2 >> 31 == 1;

    let diff_pre = number1.overflowing_sub(number2);
    let diff = diff_pre.0.overflowing_sub(u32::from(!cpu.status_register.carry));
    let diff_negative = diff.0 >> 31 == 1;

    cpu.status_register.carry = diff.1;
    cpu.status_register.overflow = (number1_negative != number2_negative) && (diff_negative != number1_negative);
    cpu.status_register.zero = diff.0 == 0;
    cpu.status_register.negative = diff_negative;
    cpu.registers.r0 = diff.0;

    Ok(())
}

// u8 <-> i8 conversion is intended, see comment below
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub fn asr(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    // For this instruction, we convert the 32-bit word to a signed integer and then do an arithmetic
    // shift right on that (>> does ASR on i32, LSR on u32), and then convert it back to an 32-bit
    // word and update memory.
    // c.f. https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators
    let word = operands[0].read_word()?;
    let word = word as i32;
    let word = word >> 1;
    let word = word as u32;
    operands[0].write_word(cpu, word)
}
