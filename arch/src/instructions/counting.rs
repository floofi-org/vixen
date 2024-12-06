use crate::core::instruction::mode::Addressing;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn inc(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::ZeroPage | Addressing::Absolute = mode {
        let initial = operands[0].read_word()?;
        let result = initial.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = initial == 127;
        operands[0].write_word(cpu, result.0)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dec(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::ZeroPage | Addressing::Absolute = mode {
        let initial = operands[0].read_word()?;
        let result = initial.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = initial == 128;
        cpu.status_register.zero = result.0 == 0;
        operands[0].write_word(cpu, result.0)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ina(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.a.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.a == 127;
        cpu.registers.a = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dea(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.a.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.a == 128;
        cpu.status_register.zero = result.0 == 0;
        cpu.registers.a = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn inx(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.x.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.x == 127;
        cpu.registers.x = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dex(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.x.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.x == 128;
        cpu.status_register.zero = result.0 == 0;
        cpu.registers.x = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn iny(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.y.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.y == 127;
        cpu.registers.y = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dey(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.y.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.y == 128;
        cpu.status_register.zero = result.0 == 0;
        cpu.registers.y = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
