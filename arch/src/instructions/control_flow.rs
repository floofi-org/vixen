use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::cpu::SystemStack;
use crate::InstructionResult;

pub fn jmp(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let position = operands[0].get_address()?;
    cpu.system_stack_save_state()?;
    cpu.program_counter = position - 15;
    Ok(())
}

pub fn jmpl(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    let position = operands[0].get_address()?;
    cpu.program_counter = position - 15;
    Ok(())
}

pub fn ret(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.system_stack_restore_state()
}

pub fn jz(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.zero {
        jmpl(operands, cpu)
    } else {
        Ok(())
    }
}

pub fn jnz(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.zero {
        Ok(())
    } else {
        jmpl(operands, cpu)
    }
}

pub fn jc(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.carry {
        jmpl(operands, cpu)
    } else {
        Ok(())
    }
}

pub fn jnc(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.carry {
        Ok(())
    } else {
        jmpl(operands, cpu)
    }
}

pub fn jo(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.overflow {
        jmpl(operands, cpu)
    } else {
        Ok(())
    }
}

pub fn jno(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.overflow {
        Ok(())
    } else {
        jmpl(operands, cpu)
    }
}

pub fn int(_operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    // They literally are just 8-bit binary numbers
    #[allow(clippy::unreadable_literal)]
    Err(match cpu.registers.r13 & 0b1111 {
        0x0 => Interrupt::User1,
        0x1 => Interrupt::User2,
        0x2 => Interrupt::User3,
        0x3 => Interrupt::User4,
        0x4 => Interrupt::User5,
        0x5 => Interrupt::User6,
        0x6 => Interrupt::User7,
        0x7 => Interrupt::User8,
        0x8 => Interrupt::User9,
        0x9 => Interrupt::User10,
        0xA => Interrupt::User11,
        0xB => Interrupt::User12,
        0xC => Interrupt::User13,
        0xD => Interrupt::User14,
        0xE => Interrupt::User15,
        _ => Interrupt::User16
    })
}

pub fn iret(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.interrupt = false;
    cpu.status_register.double_fault = false;
    ret(operands, cpu)?;
    cpu.program_counter -= 15;
    Ok(())
}

pub fn irets(operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    cpu.status_register.interrupt = false;
    cpu.status_register.double_fault = false;
    ret(operands, cpu)
}

pub fn nop(_operands: &[Operand; 3], _cpu: &mut CPU) -> InstructionResult {
    Ok(())
}

#[allow(clippy::empty_loop)]
pub fn hlt(_operands: &[Operand; 3], _cpu: &mut CPU) -> InstructionResult {
    loop {}
}

pub fn js(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.negative {
        jmpl(operands, cpu)
    } else {
        Ok(())
    }
}

pub fn jns(operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if cpu.status_register.negative {
        jmpl(operands, cpu)
    } else {
        Ok(())
    }
}
