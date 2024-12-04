use std::str::Chars;
use std::collections::VecDeque;

pub struct Scanner<'a> {
    source: Chars<'a>,
    peeked: VecDeque<char>,
    position: Position,
}

#[derive(Debug)]
pub enum TokenType {
    Colon,
    Semicolon,
    Comma,
    LineBreak,
    Dot,
    EOF,
    Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    Number(u8),
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    position: Position,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let source = source.chars();
        let peeked = VecDeque::new();
        let position = Default::default();

        Self {
            source,
            peeked,
            position,
        }
    }

    pub fn scan(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match Token::scan(&mut self) {
                Some(Token { token_type: TokenType::EOF, position: _ }) => break,
                Some(token) => tokens.push(token),
                None => {},
            }
        }

        tokens
    }

    fn next(&mut self) -> Option<char> {
        self.position.column += 1;

        if let Some(char) = self.peeked.pop_front() {
            return Some(char);
        }

        self.source.next()
    }

    fn next_word(&mut self) -> Option<String> {
        let mut word = String::new();

        while let Some(character) = self.peek() {
            if character.is_ascii_whitespace() {
                break;
            }

            // Not sure how to get rid of peek -> next
            let character = self.next().unwrap();
            word.push(character);
        }


        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    fn peek(&mut self) -> Option<char> {
        let char = self.source.next()?;
        self.peeked.push_back(char);

        Some(char)
    }

    fn skip_line(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_ascii_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }
}

impl Token {
    fn scan(scanner: &mut Scanner) -> Option<Token> {
        let character = if let Some(c) = scanner.peek() {
            c
        } else {
            return Some(Token {
                token_type: TokenType::EOF,
                position: scanner.position.clone(),
            });
        };

        let token_type = match character {
            ':' => Self::consuming(scanner, TokenType::Colon),
            ',' => Self::consuming(scanner, TokenType::Comma),
            '.' => Self::consuming(scanner, TokenType::Dot),

            // Comment
            ';' => {
                scanner.skip_line();
                return None;
            },

            // Ideally this should be greedy, i.e consecutive whitespace yield exactly one token
            // Also umm handle Windows' \r\n failure of a design
            '\n' => {
                scanner.position.column = 0;
                scanner.position.line += 1;
                Self::consuming(scanner, TokenType::LineBreak)
            },

            '0'..='9' => {
                let number = scanner.next_word()?;
                let number = number.parse().expect("Your number is silly");

                TokenType::Literal(Literal::Number(number))
            },

            ' ' => {
                return None;
            },

            ident => {
                let ident = scanner.next_word()?;

                TokenType::Literal(Literal::Identifier(ident))
            },
        };

        let token = Token {
            token_type,
            position: scanner.position.clone(),
        };

        Some(token)
    }

    fn consuming(scanner: &mut Scanner, token_type: TokenType) -> TokenType {
        scanner.next();
        token_type
    }
}
