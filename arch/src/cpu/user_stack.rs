use crate::core::Interrupt;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::CPUResult;

pub trait UserStack {
    fn user_stack_push_word(&mut self, value: u32) -> CPUResult<()>;
    fn user_stack_pull_word(&mut self) -> CPUResult<u32>;
    fn user_stack_save_state(&mut self) -> CPUResult<()>;
    fn user_stack_restore_state(&mut self) -> CPUResult<()>;
    fn user_stack_pull_state(&mut self) -> CPUResult<(u32, StatusRegister)>;
}

impl UserStack for CPU {
    fn user_stack_push_word(&mut self, value: u32) -> CPUResult<()> {
        if self.stack_pointer <= 0x0000_0004 {
            Err(Interrupt::StackOverflow)
        } else {
            let bytes = value.to_le_bytes();
            self.memory[(self.stack_pointer as usize)..(self.stack_pointer as usize + 4)].copy_from_slice(&bytes);
            self.stack_pointer += 4;
            Ok(())
        }
    }

    fn user_stack_pull_word(&mut self) -> CPUResult<u32> {
        if self.stack_pointer >= 0x1fff_fffb {
            Err(Interrupt::StackUnderflow)
        } else {
            self.stack_pointer -= 4;
            Ok(u32::from_le_bytes([
                self.memory[self.stack_pointer as usize],
                self.memory[(self.stack_pointer + 1) as usize],
                self.memory[(self.stack_pointer + 2) as usize],
                self.memory[(self.stack_pointer + 3) as usize]
            ]))
        }
    }

    fn user_stack_save_state(&mut self) -> CPUResult<()> {
        let sr: u8 = self.status_register.into();
        self.user_stack_push_word(u32::from(sr))?;
        self.user_stack_push_word(self.program_counter)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn user_stack_restore_state(&mut self) -> CPUResult<()> {
        self.program_counter = self.user_stack_pull_word()?;
        self.status_register = StatusRegister::from(self.user_stack_pull_word()? as u8);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn user_stack_pull_state(&mut self) -> CPUResult<(u32, StatusRegister)> {
        let program_counter = self.user_stack_pull_word()?;
        let status_register = StatusRegister::from(self.user_stack_pull_word()? as u8);
        Ok((program_counter, status_register))
    }
}
