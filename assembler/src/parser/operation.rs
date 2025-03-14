use vixen::core::instruction::Operation;

use super::ParseError;

#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! operation {
    ($string: expr, $( $name: expr => $operation: ident ),+ $(,)?) => {
        {
            match $string.as_str() {
            $(
                $name => Ok(Operation::$operation),
            )+
                _ => Err(ParseError::InvalidInstruction($string))
            }
        }
    };
}

#[allow(clippy::module_name_repetitions)]
pub trait OperationExt: Sized {
    fn parse(name: String) -> Result<Self, ParseError>;
}

impl OperationExt for Operation {
    fn parse(mut name: String) -> Result<Self, ParseError> {
        name.make_ascii_lowercase();

        operation! {
            name,

            "add" => Add,
            "sub" => Sub,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "sqrt" => Sqrt,
            "cbrt" => Cbrt,
            "sqre" => Sqre,
            "cube" => Cube,
            "min" => Min,
            "max" => Max,
            "addc" => Addc,
            "subc" => Subc,
            "sar" => Sar,
            "sal" => Sal,


            "and" => And,
            "or" => Or,
            "xor" => Xor,
            "nor" => Nor,
            "nand" => Nand,
            "imp" => Imp,
            "not" => Not,
            "shl" => Shl,
            "shr" => Shr,
            "rol" => Rol,
            "ror" => Ror,


            "inc" => Inc,
            "dec" => Dec,


            "cmp" => Cmp,
            "lte" => Lte,
            "gte" => Gte,
            "setz" => Setz,
            "setc" => Setc,
            "seto" => Seto,


            "mov" => Mov,
            "xchg" => Xchg,
            "clr" => Clr,
            "stc" => Stc,
            "clc" => Clc,
            "sti" => Sti,
            "cli" => Cli,
            "clv" => Clv,


            "jmp" => Jmp,
            "jmpl" => Jmpl,
            "ret" => Ret,
            "jz" => Jz,
            "jnz" => Jnz,
            "jc" => Jc,
            "jnc" => Jnc,
            "jo" => Jo,
            "jno" => Jno,
            "int" => Int,
            "iret" => Iret,
            "irets" => Irets,
            "nop" => Nop,
            "hlt" => Hlt,
            "js" => Js,
            "jns" => Jns,


            "push" => Push,
            "pop" => Pop,
            "pushf" => Pushf,
            "popf" => Popf,

            // Aliases for old mnemonics
            "jsr" => Jmp,
            "sqt" => Sqrt,
            "cbt" => Cbrt,
            "sqr" => Sqre,
            "cbe" => Cube,
            "adc" => Addc,
            "sbc" => Subc,
            "asr" => Sar,
            "asl" => Sal,
            "nad" => Nand,
            "irj" => Irets,
            "srz" => Setz,
            "src" => Setc,
            "sro" => Seto,
            "swp" => Xchg,
            "sec" => Stc,
            "sei" => Sti,
            "beq" => Jz,
            "bne" => Jnz,
            "bec" => Jc,
            "bnc" => Jnc,
            "beo" => Jo,
            "bno" => Jno,
            "irt" => Iret,
            "jam" => Hlt,
            "bpl" => Js,
            "bmi" => Jns,
            "psh" => Push,
            "pll" => Pop,
            "php" => Pushf,
            "plp" => Popf,
        }
    }
}
