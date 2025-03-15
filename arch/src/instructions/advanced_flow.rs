use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn jnae(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)?;
    let val2 = operands[1].read_word(cpu)?;
    let position = operands[2].get_address()?;

    if val1 < val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

pub fn jae(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)?;
    let val2 = operands[1].read_word(cpu)?;
    let position = operands[2].get_address()?;

    if val1 >= val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

pub fn jna(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)?;
    let val2 = operands[1].read_word(cpu)?;
    let position = operands[2].get_address()?;

    if val1 <= val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

pub fn ja(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)?;
    let val2 = operands[1].read_word(cpu)?;
    let position = operands[2].get_address()?;

    if val1 > val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn jl(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)? as i32;
    let val2 = operands[1].read_word(cpu)? as i32;
    let position = operands[2].get_address()?;

    if val1 < val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn jge(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)? as i32;
    let val2 = operands[1].read_word(cpu)? as i32;
    let position = operands[2].get_address()?;

    if val1 >= val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn jle(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)? as i32;
    let val2 = operands[1].read_word(cpu)? as i32;
    let position = operands[2].get_address()?;

    if val1 <= val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn jg(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let val1 = operands[0].read_word(cpu)? as i32;
    let val2 = operands[1].read_word(cpu)? as i32;
    let position = operands[2].get_address()?;

    if val1 > val2 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn jp(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value = operands[0].read_word(cpu)? as i32;
    let position = operands[1].get_address()?;

    if value % 2 == 0 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn jnp(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let value = operands[0].read_word(cpu)? as i32;
    let position = operands[1].get_address()?;

    if value % 2 == 1 {
        cpu.program_counter = position - 15;
        Ok(())
    } else {
        Ok(())
    }
}