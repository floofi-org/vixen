use std::fmt::{Display, Formatter};
use crate::InstructionMode::{Absolute, Immediate, Implied, ZeroPage};

pub type CPUResult<T> = Result<T, Interrupt>;
pub struct ExtractedBinaryData<'a>(&'a [u8]);

impl<'a> Display for ExtractedBinaryData<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::new();
        for (index, byte) in self.0.iter().enumerate() {
            hex.push_str(&format!("{:0>2X}", byte));
            if index % 1 == 0 {
                hex.push(' ');
            }
        }
        write!(f, "{}", hex.trim())
    }
}

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

#[derive(Debug)]
pub struct CPUStatusRegister {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool
}

impl Display for CPUStatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{zero}{carry}{overflow}",
            zero = if self.zero {
                "z"
            } else {
                "-"
            },
            carry = if self.carry {
                "c"
            } else {
                "-"
            },
            overflow = if self.overflow {
                "o"
            } else {
                "-"
            }
        )
    }
}

#[derive(Debug)]
struct CPUMemory {
    zeropage: [u8; 256],
    stack: [u8; 256],
    ram: [u8; 32256],
    io: [u8; 24576],
    rom: [u8; 7936],
    cpuinfo: [u8; 256]
}

#[derive(Debug)]
pub struct Instruction<'a> {
    operation: InstructionOperation,
    mode: InstructionMode,
    operands: [Operand<'a>; 2]
}

#[derive(Debug)]
pub enum Operand<'a> {
    Literal(u16),
    Register(RegisterId, &'a u8),
    ZeroPage(u16, &'a u8),
    Memory(u16, &'a u8)
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterId {
    A, X, Y,
    R0, R1, R2, R3, R4, R5, R6, R7
}

impl RegisterId {
    fn try_from(value: u16) -> CPUResult<Self> {
        match value {
            0x0001 => Ok(Self::A),
            0x0011 => Ok(Self::X),
            0x0012 => Ok(Self::Y),
            0x1000 => Ok(Self::R0),
            0x1001 => Ok(Self::R1),
            0x1002 => Ok(Self::R2),
            0x1003 => Ok(Self::R3),
            0x1004 => Ok(Self::R4),
            0x1005 => Ok(Self::R5),
            0x1006 => Ok(Self::R6),
            0x1007 => Ok(Self::R7),
            _ => Err(Interrupt::IllegalMemory)
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    pub registers: CPURegisters,
    pub sp: u8,
    pub pc: u16,
    pub sr: CPUStatusRegister,
    pub memory: [u8; 0xFFFF]
}

#[derive(Debug, Clone, Copy)]
enum InstructionMode {
    Immediate,
    Implied,
    ZeroPage,
    Absolute
}

#[derive(Debug, Clone, Copy)]
enum InstructionOperation {
    /* 0x01?? */ Add, Sub, Mul, Div, Mod, Sqt, Cbt, Sqr, Cbe, Min, Max,
    /* 0x02?? */ And, Or,  Xor, Nor, Nad, Imp, Not, Shl, Shr, Rol, Ror,
    /* 0x03?? */ Inc, Dec, Ina, Dea, Inx, Dex, Iny, Dey,
    /* 0x04?? */ Cmp, Cpx, Cpy, Lte, Gte, Sez, Sec, Seo,
    /* 0x05?? */ Lda, Ldx, Ldy, Ldz, Sta, Stx, Sty, Mov, Swp, Clr,
    /* 0x06?? */ Jmp, Jsr, Ret, Beq, Bne, Bec, Bnc, Beo, Bno, Int, Irt, Nop,
    /* 0x07?? */ Pha, Pla, Phx, Plx, Phy, Ply, Psh, Pll
}

impl InstructionOperation {
    fn try_from(value: u16) -> CPUResult<Self> {
        match value {
            // 0x01?? - Arithmetic and Algebric Instructions
            0x010 => Ok(Self::Add),
            0x011 => Ok(Self::Sub),
            0x012 => Ok(Self::Mul),
            0x013 => Ok(Self::Div),
            0x014 => Ok(Self::Mod),
            0x015 => Ok(Self::Sqt),
            0x016 => Ok(Self::Cbt),
            0x017 => Ok(Self::Sqr),
            0x018 => Ok(Self::Cbe),
            0x019 => Ok(Self::Min),
            0x01A => Ok(Self::Max),

            // 0x02?? - Logic Instructions
            0x020 => Ok(Self::And),
            0x021 => Ok(Self::Or),
            0x022 => Ok(Self::Xor),
            0x023 => Ok(Self::Nor),
            0x024 => Ok(Self::Nad),
            0x025 => Ok(Self::Imp),
            0x026 => Ok(Self::Not),
            0x027 => Ok(Self::Shl),
            0x028 => Ok(Self::Shr),
            0x029 => Ok(Self::Rol),
            0x02A => Ok(Self::Ror),

            // 0x03?? - Counting Instructions
            0x030 => Ok(Self::Inc),
            0x031 => Ok(Self::Dec),
            0x032 => Ok(Self::Ina),
            0x033 => Ok(Self::Dea),
            0x034 => Ok(Self::Inx),
            0x035 => Ok(Self::Dex),
            0x036 => Ok(Self::Iny),
            0x037 => Ok(Self::Dey),

            // 0x04?? - Comparison Instructions
            0x040 => Ok(Self::Cmp),
            0x041 => Ok(Self::Cpx),
            0x042 => Ok(Self::Cpy),
            0x043 => Ok(Self::Lte),
            0x044 => Ok(Self::Gte),
            0x045 => Ok(Self::Sez),
            0x046 => Ok(Self::Sec),
            0x047 => Ok(Self::Seo),

            // 0x05?? - Data Movement Instructions
            0x050 => Ok(Self::Lda),
            0x051 => Ok(Self::Ldx),
            0x052 => Ok(Self::Ldy),
            0x053 => Ok(Self::Ldz),
            0x054 => Ok(Self::Sta),
            0x055 => Ok(Self::Stx),
            0x056 => Ok(Self::Sty),
            0x057 => Ok(Self::Mov),
            0x058 => Ok(Self::Swp),
            0x059 => Ok(Self::Clr),

            // 0x06?? - Control Flow Instructions
            0x060 => Ok(Self::Jmp),
            0x061 => Ok(Self::Jsr),
            0x062 => Ok(Self::Ret),
            0x063 => Ok(Self::Beq),
            0x064 => Ok(Self::Bne),
            0x065 => Ok(Self::Bec),
            0x066 => Ok(Self::Bnc),
            0x067 => Ok(Self::Beo),
            0x068 => Ok(Self::Bno),
            0x069 => Ok(Self::Int),
            0x06A => Ok(Self::Irt),
            0x06B => Ok(Self::Nop),

            // 0x07?? - Stack Instructions
            0x070 => Ok(Self::Pha),
            0x071 => Ok(Self::Pla),
            0x072 => Ok(Self::Phx),
            0x073 => Ok(Self::Plx),
            0x074 => Ok(Self::Phy),
            0x075 => Ok(Self::Ply),
            0x076 => Ok(Self::Psh),
            0x077 => Ok(Self::Pll),

            // Illegal instruction
            _ => Err(Interrupt::IllegalInstruction)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    Rtc, AsyncIO, Hardware, External,
    Breakpoint, IllegalInstruction, IllegalMemory, DivideByZero,
    StackOverflow, StackUnderflow,
    User1, User2, User3, User4, User5, User6, User7, User8,
    User9, User10, User11, User12, User13, User14, User15, User16,
    Failure, Reset
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Interrupt::Rtc => write!(f, "0x00 (Real-time clock tick)"),
            Interrupt::AsyncIO => write!(f, "0x01 (Asynchronous I/O event)"),
            Interrupt::Hardware => write!(f, "0x02 (General hardware fault)"),
            Interrupt::External => write!(f, "0x03 (External hardware interrupt)"),
            Interrupt::Breakpoint => write!(f, "0x10 (Breakpoint hit)"),
            Interrupt::IllegalInstruction => write!(f, "0x11 (Illegal instruction)"),
            Interrupt::IllegalMemory => write!(f, "0x12 (Illegal memory access)"),
            Interrupt::DivideByZero => write!(f, "0x13 (Divide by zero)"),
            Interrupt::StackOverflow => write!(f, "0x20 (Stack overflow)"),
            Interrupt::StackUnderflow => write!(f, "0x21 (Stack underflow)"),
            Interrupt::User1 => write!(f, "0xE0 (User-defined interrupt 1)"),
            Interrupt::User2 => write!(f, "0xE1 (User-defined interrupt 2)"),
            Interrupt::User3 => write!(f, "0xE2 (User-defined interrupt 3)"),
            Interrupt::User4 => write!(f, "0xE3 (User-defined interrupt 4)"),
            Interrupt::User5 => write!(f, "0xE4 (User-defined interrupt 5)"),
            Interrupt::User6 => write!(f, "0xE5 (User-defined interrupt 6)"),
            Interrupt::User7 => write!(f, "0xE6 (User-defined interrupt 7)"),
            Interrupt::User8 => write!(f, "0xE7 (User-defined interrupt 8)"),
            Interrupt::User9 => write!(f, "0xE8 (User-defined interrupt 9)"),
            Interrupt::User10 => write!(f, "0xE9 (User-defined interrupt 10)"),
            Interrupt::User11 => write!(f, "0xEA (User-defined interrupt 11)"),
            Interrupt::User12 => write!(f, "0xEB (User-defined interrupt 12)"),
            Interrupt::User13 => write!(f, "0xEC (User-defined interrupt 13)"),
            Interrupt::User14 => write!(f, "0xED (User-defined interrupt 14)"),
            Interrupt::User15 => write!(f, "0xEE (User-defined interrupt 15)"),
            Interrupt::User16 => write!(f, "0xEF (User-defined interrupt 16)"),
            Interrupt::Failure => write!(f, "0xFE (Internal system failure)"),
            Interrupt::Reset => write!(f, "0xFF (System reset)")
        }
    }
}

impl InstructionMode {
    fn try_from(value: u8) -> CPUResult<Self> {
        match value {
            0 => Ok(Immediate),
            1 => Ok(Implied),
            2 => Ok(ZeroPage),
            3 => Ok(Absolute),
            _ => Err(Interrupt::IllegalInstruction)
        }
    }
}

impl CPU {
    pub fn new(rom: &[u8]) -> Self {
        let mut cpu = Self {
            registers: CPURegisters::new(),
            sp: 0,
            pc: 0xE000,
            sr: CPUStatusRegister::new(),
            memory: [0; 0xFFFF]
        };
        for (index, value) in cpu.memory.iter_mut().enumerate() {
            if 0xE000 <= index && index <= 0xFF00 && index - 0xE000 < rom.len() {
                *value = rom[index - 0xE000];
            }
        }
        cpu
    }

    pub fn get_register(&self, id: RegisterId) -> &u8 {
        match id {
            RegisterId::A => &self.registers.a,
            RegisterId::X => &self.registers.x,
            RegisterId::Y => &self.registers.y,
            RegisterId::R0 => &self.registers.r0,
            RegisterId::R1 => &self.registers.r1,
            RegisterId::R2 => &self.registers.r2,
            RegisterId::R3 => &self.registers.r3,
            RegisterId::R4 => &self.registers.r4,
            RegisterId::R5 => &self.registers.r5,
            RegisterId::R6 => &self.registers.r6,
            RegisterId::R7 => &self.registers.r7
        }
    }

    pub fn extract_instruction(&self, position: u16) -> ExtractedBinaryData {
        let index = position as usize;
        ExtractedBinaryData(&self.memory[index..index + 6])
    }

    pub fn read_instruction(&self, position: u16) -> CPUResult<Instruction> {
        let opcode = self.extract_instruction(position).0;
        let family = opcode[5];
        let operation = opcode[4] >> 4;
        let instruction = family as u16 * 0x10 + operation as u16;
        let mode = InstructionMode::try_from(opcode[4] & 0x0F)?;
        let instruction = InstructionOperation::try_from(instruction)?;

        let raw_operand1 = opcode[3] as u16 * 0x100 + opcode[2] as u16;
        let raw_operand2 = opcode[1] as u16 * 0x100 + opcode[0] as u16;

        let (operand1, operand2) = match &mode {
            Immediate => Ok((Operand::Literal(raw_operand1), Operand::Literal(raw_operand2))),
            Implied => {
                let register1 = RegisterId::try_from(raw_operand1)?;
                let register2 = RegisterId::try_from(raw_operand2)?;
                Ok((
                    Operand::Register(register1, self.get_register(register1)),
                    Operand::Register(register2, self.get_register(register2))
                ))
            },
            ZeroPage => {
                if raw_operand1 > 0xFF || raw_operand2 > 0xFF {
                    Err(Interrupt::IllegalMemory)
                } else {
                    let operand1 = Operand::ZeroPage(raw_operand1, &self.memory[raw_operand1 as usize]);
                    let operand2 = Operand::ZeroPage(raw_operand2, &self.memory[raw_operand2 as usize]);
                    Ok((operand1, operand2))
                }
            },
            Absolute => {
                if raw_operand1 <= 0xFF || raw_operand2 <= 0xFF {
                    Err(Interrupt::IllegalMemory)
                } else {
                    let operand1 = Operand::Memory(raw_operand1, &self.memory[raw_operand1 as usize]);
                    let operand2 = Operand::Memory(raw_operand2, &self.memory[raw_operand2 as usize]);
                    Ok((operand1, operand2))
                }
            }
        }?;

        Ok(Instruction {
            operation: instruction,
            mode,
            operands: [operand1, operand2],
        })
    }
}

impl CPURegisters {
    fn new() -> Self {
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

impl CPUStatusRegister {
    fn new() -> Self {
        Self {
            zero: false,
            carry: false,
            overflow: false,
        }
    }
}