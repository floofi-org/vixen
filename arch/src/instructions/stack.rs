use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::cpu::user_stack::UserStack;
use crate::InstructionResult;

pub fn pha(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.a)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pla(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.a = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phx(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.x)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plx(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.x = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phy(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.y)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ply(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.y = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn psh(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        cpu.user_stack_push_word(operands[0].read_word()?)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pll(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Absolute = mode {
        let word = cpu.user_stack_pull_word()?;
        operands[0].write_word(cpu, word)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
