use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::InstructionResult;

pub fn and(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = number1 & number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn or(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = number1 | number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn xor(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = number1 ^ number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn nor(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = !(number1 | number2);

        cpu.status_register.zero = result == 0;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn nad(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = !(number1 & number2);

        cpu.status_register.zero = result == 0;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn imp(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Immediate | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = (!number1) | number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.a = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn not(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.registers.a = !cpu.registers.a;
        cpu.status_register.zero = cpu.registers.a == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn shl(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Direct | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number << 1;

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn shr(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Direct | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number >> 1;

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn rol(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Direct | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number.rotate_left(1);

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ror(mode: InstructionMode, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Direct | InstructionMode::ZeroPage | InstructionMode::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number.rotate_right(1);

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
