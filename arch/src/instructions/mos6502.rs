use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::cpu::UserStack;
use crate::InstructionResult;
use crate::instructions::control_flow::jmp;

pub fn bpl(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.negative {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bmi(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.negative {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn adc(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number1_negative = number1 >> 7 == 1;
        let number2 = operands[1].read_word()?;
        let number2_negative = number2 >> 7 == 1;

        let sum_pre = number1.overflowing_add(number2);
        let sum = sum_pre.0.overflowing_add(u8::from(cpu.status_register.carry));
        let sum_negative = sum_pre.0 >> 7 == 1;

        cpu.status_register.carry = sum.1;
        cpu.status_register.overflow = (number1_negative == number2_negative) && (sum_negative != number1_negative);
        cpu.status_register.zero = sum.0 == 0;
        cpu.status_register.negative = sum_negative;
        cpu.registers.a = sum.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sbc(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::ZeroPage | Addressing::Relative = mode {
        let number1 = operands[0].read_word()?;
        let number1_negative = number1 >> 7 == 1;
        let number2 = operands[1].read_word()?;
        let number2_negative = number2 >> 7 == 1;

        let diff_pre = number1.overflowing_sub(number2);
        let diff = diff_pre.0.overflowing_sub(u8::from(!cpu.status_register.carry));
        let diff_negative = diff.0 >> 7 == 1;

        cpu.status_register.carry = diff.1;
        cpu.status_register.overflow = (number1_negative != number2_negative) && (diff_negative != number1_negative);
        cpu.status_register.zero = diff.0 == 0;
        cpu.status_register.negative = diff_negative;
        cpu.registers.a = diff.0;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bit(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    #[allow(clippy::unreadable_literal)]
    // They literally are just 8-bit binary numbers
    if let Addressing::ZeroPage | Addressing::Absolute = mode {
        let result = cpu.registers.a & operands[0].read_word()?;
        cpu.status_register.negative = (result & 0b10000000) == 0b10000000;
        cpu.status_register.overflow = (result & 0b01000000) == 0b01000000;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
// u8 <-> i8 conversion is intended, see comment below
pub fn asr(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::ZeroPage | Addressing::Absolute = mode {
        // For this instruction, we convert the 8-bit word to a signed integer and then do an arithmetic
        // shift right on that (>> does ASR on i8, LSR on u8), and then convert it back to an 8-bit
        // word and update memory.
        // c.f. https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators
        let word = operands[0].read_word()?;
        let word = word as i8;
        let word = word >> 1;
        let word = word as u8;
        operands[0].write_word(cpu, word)
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sec(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.carry = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clc(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.carry = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sei(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.interrupt_disable = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cli(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.interrupt_disable = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clv(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.overflow = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}


pub fn php(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.status_register.into())?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plp(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let word = cpu.user_stack_pull_word()?;
        cpu.status_register = StatusRegister::from(word);
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
