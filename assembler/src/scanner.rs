use std::str::Chars;
use std::iter::Peekable;

mod token;

use crate::models::{Location, Token, TokenWithSpan};
pub use token::UnexpectedToken;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    location: Location,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        let source = source.chars().peekable();
        let location = Location::default();

        Self {
            source,
            location,
        }
    }

    #[must_use]
    pub fn scan(mut self) -> Result<Vec<TokenWithSpan>, UnexpectedToken> {
        let mut tokens = Vec::new();

        loop {
            let Some(token) = TokenWithSpan::scan(&mut self)? else {
                continue;
            };

            let eof = matches!(token.token, Token::EOF);
            tokens.push(token);

            if eof {
                break;
            }
        }

        Ok(tokens)
    }

    fn skip_while(&mut self, mut f: impl FnMut(&char) -> bool) {
        while let Some(c) = self.peek() {
            if f(c) {
                self.next();
            } else {
                break;
            }
        }
    }

    fn next_while(&mut self, mut f: impl FnMut(&char) -> bool) -> Option<String> {
        let string: String = std::iter::from_fn(|| self.next_if(&mut f)).collect();

        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    }

    fn next_if(&mut self, f: impl FnOnce(&char) -> bool) -> Option<char> {
        let char = self.source.next_if(f)?;
        self.location.column += 1;

        Some(char)
    }

    fn next(&mut self) -> Option<char> {
        let char = self.source.next()?;
        self.location.column += 1;

        Some(char)
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
}
