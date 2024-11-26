pub mod stack;
pub mod decoder;

use crate::core::register_id::RegisterId;
use crate::core::registers::Registers;
use crate::core::status_register::StatusRegister;
use crate::cpu::stack::Stack;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub sp: u16,
    pub pc: u16,
    pub sr: StatusRegister,
    pub memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new(rom: &[u8]) -> Self {
        let mut cpu = Self {
            registers: Registers::default(),
            sp: 0x0100,
            pc: 0xE000,
            sr: StatusRegister::default(),
            memory: [0; 0xFFFF]
        };
        /*for (index, value) in cpu.memory.iter_mut().enumerate() {
            if 0xE000 <= index && index <= 0xFF00 && index - 0xE000 < rom.len() {
                *value = rom[index - 0xE000];
            }
        }*/
        cpu.memory[0xE000..(0xE000 + rom.len())].copy_from_slice(rom);
        cpu.stack_push_dword(0xE000).unwrap();
        cpu
    }

    pub fn get_register(&self, register_id: RegisterId) -> u8 {
        match register_id {
            RegisterId::A => self.registers.a,
            RegisterId::X => self.registers.x,
            RegisterId::Y => self.registers.y,
            RegisterId::R0 => self.registers.r0,
            RegisterId::R1 => self.registers.r1,
            RegisterId::R2 => self.registers.r2,
            RegisterId::R3 => self.registers.r3,
            RegisterId::R4 => self.registers.r4,
            RegisterId::R5 => self.registers.r5,
            RegisterId::R6 => self.registers.r6,
            RegisterId::R7 => self.registers.r7
        }
    }
}
