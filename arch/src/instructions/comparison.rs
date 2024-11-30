use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn cmp(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value1 = cpu.registers.a;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cpx(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value1 = cpu.registers.x;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cpy(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value1 = cpu.registers.y;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn lte(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value1 = cpu.registers.a;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 <= value2;
        cpu.status_register.carry = value1 > value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn gte(mode: InstructionMode, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::Implied | InstructionMode::ZeroPage | InstructionMode::Absolute = mode {
        let value1 = cpu.registers.a;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 >= value2;
        cpu.status_register.carry = value1 < value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn srz(mode: InstructionMode, operand: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        operand[0].write_word(cpu, if cpu.status_register.zero { 1 } else { 0 })?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn src(mode: InstructionMode, operand: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        operand[0].write_word(cpu, if cpu.status_register.carry { 1 } else { 0 })?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sro(mode: InstructionMode, operand: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        operand[0].write_word(cpu, if cpu.status_register.overflow { 1 } else { 0 })?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}