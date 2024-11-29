use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn lda(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        cpu.registers.a = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.a == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ldx(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        cpu.registers.x = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.x == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ldy(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        cpu.registers.y = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.y == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ldz(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        operands[0].write_word(cpu, 0)?;
        cpu.status_register.zero = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sta(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        operands[0].write_word(cpu, cpu.registers.a)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn stx(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        operands[0].write_word(cpu, cpu.registers.x)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sty(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        operands[0].write_word(cpu, cpu.registers.y)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn mov(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value = operands[0].read_word()?;
        operands[1].write_word(cpu, value)?;
        operands[0].write_word(cpu, 0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn swp(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value1 = operands[0].read_word()?;
        let value2 = operands[1].read_word()?;
        operands[1].write_word(cpu, value1)?;
        operands[0].write_word(cpu, value2)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clr(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        operands[0].write_word(cpu, 0)?;
        cpu.status_register.zero = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}