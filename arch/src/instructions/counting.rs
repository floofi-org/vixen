use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn inc(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
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
    if let Addressing::Absolute = mode {
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
        let result = cpu.registers.r0.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.r0 == 127;
        cpu.registers.r0 = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dea(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.r0.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.r0 == 128;
        cpu.status_register.zero = result.0 == 0;
        cpu.registers.r0 = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn inx(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.r1.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.r1 == 127;
        cpu.registers.r1 = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dex(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.r1.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.r1 == 128;
        cpu.status_register.zero = result.0 == 0;
        cpu.registers.r1 = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn iny(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.r2.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.r2 == 127;
        cpu.registers.r2 = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dey(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let result = cpu.registers.r2.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = cpu.registers.r2 == 128;
        cpu.status_register.zero = result.0 == 0;
        cpu.registers.r2 = result.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
