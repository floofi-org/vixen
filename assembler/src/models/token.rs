mod span;
mod macros;

#[allow(clippy::module_name_repetitions)]
pub use span::TokenWithSpan;

use macros::{token, tokens};


#[allow(clippy::module_name_repetitions)]
pub trait FromToken: Sized {
    fn from_token(token: Token) -> Option<Self>;
}

#[derive(Debug)]
pub enum Token {
    Colon,
    Hash,
    Minus,
    Plus,
    Dot,
    Comma,
    LineBreak,
    EOF,
    Identifier(String),
    Number(u16),
}

tokens! {
    Colon,
    Hash,
    Minus,
    Plus,
    Dot,
    Comma,
    LineBreak,
    EOF,
    Identifier(string: String),
    Number(number: u16)
}
