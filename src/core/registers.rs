#[derive(Debug)]
pub struct CPURegisters {
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

impl CPURegisters {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
        }
    }
}