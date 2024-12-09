use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn and(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = number1 & number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.r0 = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn or(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = number1 | number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.r0 = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn xor(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = number1 ^ number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.r0 = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn nor(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = !(number1 | number2);

        cpu.status_register.zero = result == 0;
        cpu.registers.r0 = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn nad(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = !(number1 & number2);

        cpu.status_register.zero = result == 0;
        cpu.registers.r0 = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn imp(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number2 = operands[1].read_word()?;
        let result = (!number1) | number2;

        cpu.status_register.zero = result == 0;
        cpu.registers.r0 = result;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn not(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.r0 = !cpu.registers.r0;
        cpu.status_register.zero = cpu.registers.r0 == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn shl(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number << 1;

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn shr(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number >> 1;

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn rol(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number.rotate_left(1);

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ror(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode {
        let number = operands[0].read_word()?;
        let result = number.rotate_right(1);

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
