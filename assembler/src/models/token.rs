mod literal;
mod span;
mod macros;

#[allow(clippy::module_name_repetitions)]
pub use span::TokenWithSpan;
pub use literal::{Literal, Identifier, Number, StringLiteral};

use macros::{token, tokens};

#[allow(clippy::module_name_repetitions)]
pub trait FromToken: Sized {
    fn from_token(token: Token) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum Token {
    Colon,
    Hash,
    Equals,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Minus,
    Plus,
    Dot,
    Comma,
    LineBreak,
    EOF,
    Literal(Literal)
}

tokens! {
    Colon,
    Hash,
    Equals,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Minus,
    Plus,
    Dot,
    Comma,
    LineBreak,
    EOF
}
