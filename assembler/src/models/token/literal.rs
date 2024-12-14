use super::{FromToken, Token};

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct Number(pub u32);

#[allow(clippy::module_name_repetitions)]
pub struct StringLiteral(pub String);

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Number(u32),
    String(String),
}

impl FromToken for Literal {
    fn from_token(token: Token) -> Option<Self> {
        if let Token::Literal(literal) = token {
            Some(literal)
        } else {
            None
        }
    }
}

impl FromToken for Identifier {
    fn from_token(token: Token) -> Option<Self> {
        if let Token::Literal(Literal::Identifier(ident)) = token {
            Some(Self(ident))
        } else {
            None
        }
    }
}


impl FromToken for Number {
    fn from_token(token: Token) -> Option<Self> {
        if let Token::Literal(Literal::Number(number)) = token {
            Some(Self(number))
        } else {
            None
        }
    }
}

impl FromToken for StringLiteral {
    fn from_token(token: Token) -> Option<Self> {
        if let Token::Literal(Literal::String(string)) = token {
            Some(Self(string))
        } else {
            None
        }
    }
}
