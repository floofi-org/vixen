use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::cpu::user_stack::UserStack;
use crate::InstructionResult;

pub fn pha(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.a)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pla(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.registers.a = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phx(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.x)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plx(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.registers.x = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phy(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.y)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ply(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.registers.y = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn psh(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::Direct | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        cpu.user_stack_push_word(operands[0].read_word()?)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pll(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Direct | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let word = cpu.user_stack_pull_word()?;
        operands[0].write_word(cpu, word)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
