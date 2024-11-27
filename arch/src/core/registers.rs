pub mod register_id;
pub mod status_register;

#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub r0: u8,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
    pub r4: u8,
    pub r5: u8,
    pub r6: u8,
    pub r7: u8
}
