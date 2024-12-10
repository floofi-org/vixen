use super::{FromToken, Token};

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct Number(pub u32);

#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    Number(u32),
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
