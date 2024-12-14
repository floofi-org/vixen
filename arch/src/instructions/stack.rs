use crate::core::MemoryCell;
use crate::core::Operand;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::cpu::user_stack::UserStack;
use crate::InstructionResult;

pub fn pha(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.user_stack_push_word(cpu.registers.r0)?;
    Ok(())
}

pub fn pla(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.registers.r0 = cpu.user_stack_pull_word()?;
    Ok(())
}

pub fn phx(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.user_stack_push_word(cpu.registers.r1)?;
    Ok(())
}

pub fn plx(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.registers.r1 = cpu.user_stack_pull_word()?;
    Ok(())
}

pub fn phy(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.user_stack_push_word(cpu.registers.r2)?;
    Ok(())
}

pub fn ply(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.registers.r2 = cpu.user_stack_pull_word()?;
    Ok(())
}

pub fn psh(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.user_stack_push_word(operands[0].read_word()?)?;
    Ok(())
}

pub fn pll(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let word = cpu.user_stack_pull_word()?;
    operands[0].write_word(cpu, word)?;
    Ok(())
}

pub fn php(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let sr: u8 = cpu.status_register.into();
    cpu.user_stack_push_word(u32::from(sr))?;
    Ok(())
}

// Status register is always 8-bit
#[allow(clippy::cast_possible_truncation)]
pub fn plp(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let word = cpu.user_stack_pull_word()?;
    cpu.status_register = StatusRegister::from(word as u8);
    Ok(())
}
