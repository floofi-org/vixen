use crate::core::interrupt::Interrupt;
use crate::cpu::CPU;
use crate::CPUResult;

pub trait Stack {
    fn stack_push_word(&mut self, value: u8) -> CPUResult<()>;
    fn stack_push_dword(&mut self, value: u16) -> CPUResult<()>;
    fn stack_pull_word(&mut self) -> CPUResult<u8>;
    fn stack_pull_dword(&mut self) -> CPUResult<u16>;
}

impl Stack for CPU {
    fn stack_push_word(&mut self, value: u8) -> CPUResult<()> {
        if self.stack_pointer >= 0x01FF {
            Err(Interrupt::StackOverflow)
        } else {
            self.memory[self.stack_pointer as usize] = value;
            self.stack_pointer += 1;
            Ok(())
        }
    }

    fn stack_push_dword(&mut self, value: u16) -> CPUResult<()> {
        if self.stack_pointer >= 0x01FE {
            Err(Interrupt::StackOverflow)
        } else {
            self.memory[self.stack_pointer as usize] = (value & 0xFF) as u8;
            self.memory[(self.stack_pointer + 1) as usize] = (value >> 8) as u8;
            self.stack_pointer += 2;
            Ok(())
        }
    }

    fn stack_pull_word(&mut self) -> CPUResult<u8> {
        if self.stack_pointer <= 0x0100 {
            Err(Interrupt::StackUnderflow)
        } else {
            self.stack_pointer -= 1;
            Ok(self.memory[(self.stack_pointer + 1) as usize])
        }
    }

    fn stack_pull_dword(&mut self) -> CPUResult<u16> {
        if self.stack_pointer <= 0x0101 {
            Err(Interrupt::StackUnderflow)
        } else {
            self.stack_pointer -= 2;
            Ok((self.memory[(self.stack_pointer + 2) as usize] as u16) * 0x100 + (self.memory[(self.stack_pointer + 1) as usize] as u16))
        }
    }
}
