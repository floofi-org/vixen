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
            "sqt" => Sqt,
            "cbt" => Cbt,
            "sqr" => Sqr,
            "cbe" => Cbe,
            "min" => Min,
            "max" => Max,
            "adc" => Adc,
            "sbc" => Sbc,
            "asr" => Asr,


            "and" => And,
            "or" => Or,
            "xor" => Xor,
            "nor" => Nor,
            "nad" => Nad,
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
            "srz" => Srz,
            "src" => Src,
            "sro" => Sro,


            "ldr" => Ldr,
            "str" => Str,
            "mov" => Mov,
            "swp" => Swp,
            "clr" => Clr,
            "sec" => Sec,
            "clc" => Clc,
            "sei" => Sei,
            "cli" => Cli,
            "clv" => Clv,


            "jmp" => Jmp,
            "jsr" => Jsr,
            "ret" => Ret,
            "beq" => Beq,
            "bne" => Bne,
            "bec" => Bec,
            "bnc" => Bnc,
            "beo" => Beo,
            "bno" => Bno,
            "int" => Int,
            "irt" => Irt,
            "nop" => Nop,
            "jam" => Jam,
            "bpl" => Bpl,
            "bmi" => Bmi,


            "psh" => Psh,
            "pll" => Pll,
            "php" => Php,
            "plp" => Plp,
        }
    }
}
