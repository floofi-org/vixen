pub mod register_id;
pub mod status_register;

pub use register_id::RegisterId;
pub use status_register::StatusRegister;

#[derive(Debug, Default)]
pub struct Registers {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub r13: u32,
    pub r14: u32
}
