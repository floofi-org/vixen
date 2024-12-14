use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn and(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Immediate || mode[0] == Addressing::Relative) &&
        (mode[1] == Addressing::Immediate || mode[1] == Addressing::Relative) {
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

pub fn or(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Immediate || mode[0] == Addressing::Relative) &&
        (mode[1] == Addressing::Immediate || mode[1] == Addressing::Relative) {
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

pub fn xor(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Immediate || mode[0] == Addressing::Relative) &&
        (mode[1] == Addressing::Immediate || mode[1] == Addressing::Relative) {
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

pub fn nor(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Immediate || mode[0] == Addressing::Relative) &&
        (mode[1] == Addressing::Immediate || mode[1] == Addressing::Relative) {
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

pub fn nad(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Immediate || mode[0] == Addressing::Relative) &&
        (mode[1] == Addressing::Immediate || mode[1] == Addressing::Relative) {
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

pub fn imp(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Immediate || mode[0] == Addressing::Relative) &&
        (mode[1] == Addressing::Immediate || mode[1] == Addressing::Relative) {
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

pub fn not(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.registers.r0 = !cpu.registers.r0;
        cpu.status_register.zero = cpu.registers.r0 == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn shl(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode[0] {
        let number = operands[0].read_word()?;
        let result = number << 1;

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn shr(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode[0] {
        let number = operands[0].read_word()?;
        let result = number >> 1;

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn rol(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode[0] {
        let number = operands[0].read_word()?;
        let result = number.rotate_left(1);

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ror(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Relative = mode[0] {
        let number = operands[0].read_word()?;
        let result = number.rotate_right(1);

        cpu.status_register.zero = result == 0;
        operands[0].write_word(cpu, result)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
