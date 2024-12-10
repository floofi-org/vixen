mod literal;
mod span;
mod macros;

#[allow(clippy::module_name_repetitions)]
pub use span::TokenWithSpan;
pub use literal::{Literal, Identifier, Number};

use macros::{token, tokens};

#[allow(clippy::module_name_repetitions)]
pub trait FromToken: Sized {
    fn from_token(token: Token) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum Token {
    Colon,
    Hash,
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
    Minus,
    Plus,
    Dot,
    Comma,
    LineBreak,
    EOF
}
