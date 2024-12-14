use crate::core::Interrupt;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::CPUResult;

pub trait SystemStack {
    fn system_stack_push_word(&mut self, value: u32) -> CPUResult<()>;
    fn system_stack_pull_word(&mut self) -> CPUResult<u32>;
    fn system_stack_save_state(&mut self) -> CPUResult<()>;
    fn system_stack_restore_state(&mut self) -> CPUResult<()>;
    fn system_stack_pull_state(&mut self) -> CPUResult<(u32, StatusRegister)>;
}

impl SystemStack for CPU {
    fn system_stack_push_word(&mut self, value: u32) -> CPUResult<()> {
        if self.system_stack.len() > 256 {
            Err(Interrupt::StackOverflow)
        } else {
            self.system_stack.push(value);
            Ok(())
        }
    }

    fn system_stack_pull_word(&mut self) -> CPUResult<u32> {
        if self.system_stack.is_empty() {
            Err(Interrupt::StackUnderflow)
        } else {
            // This is intended as the system stack can store both u16 and u8 values
            #[allow(clippy::cast_possible_truncation)]
            self.system_stack.pop().ok_or(Interrupt::IllegalMemory)
        }
    }

    fn system_stack_save_state(&mut self) -> CPUResult<()> {
        let sr: u8 = self.status_register.into();
        self.system_stack_push_word(u32::from(sr))?;
        self.system_stack_push_word(self.program_counter)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn system_stack_restore_state(&mut self) -> CPUResult<()> {
        self.program_counter = self.system_stack_pull_word()?;
        self.status_register = StatusRegister::from(self.system_stack_pull_word()? as u8);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn system_stack_pull_state(&mut self) -> CPUResult<(u32, StatusRegister)> {
        let program_counter = self.system_stack_pull_word()?;
        let status_register = StatusRegister::from(self.system_stack_pull_word()? as u8);
        Ok((program_counter, status_register))
    }
}
