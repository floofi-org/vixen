use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn and(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[1].read_word(cpu)?;
    let number2 = operands[2].read_word(cpu)?;
    let result = number1 & number2;

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn or(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[1].read_word(cpu)?;
    let number2 = operands[2].read_word(cpu)?;
    let result = number1 | number2;

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn xor(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[1].read_word(cpu)?;
    let number2 = operands[2].read_word(cpu)?;
    let result = number1 ^ number2;

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn nor(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[1].read_word(cpu)?;
    let number2 = operands[2].read_word(cpu)?;
    let result = !(number1 | number2);

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn nand(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[1].read_word(cpu)?;
    let number2 = operands[2].read_word(cpu)?;
    let result = !(number1 & number2);

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn imp(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number1 = operands[1].read_word(cpu)?;
    let number2 = operands[2].read_word(cpu)?;
    let result = (!number1) | number2;

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn not(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.registers.r0 = !cpu.registers.r0;
    cpu.status_register.zero = cpu.registers.r0 == 0;
    Ok(())
}

pub fn shl(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word(cpu)?;
    let result = number << 1;

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn shr(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word(cpu)?;
    let result = number >> 1;

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn rol(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word(cpu)?;
    let result = number.rotate_left(1);

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}

pub fn ror(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let number = operands[0].read_word(cpu)?;
    let result = number.rotate_right(1);

    cpu.status_register.zero = result == 0;
    operands[0].write_word(cpu, result)?;

    Ok(())
}
