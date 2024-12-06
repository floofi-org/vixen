use super::Scanner;

mod literal;

pub use literal::Literal;

#[derive(Debug)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Default, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub begin: Location,
    pub end: Location,
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
    Literal(Literal),
}

impl TokenWithSpan {
    /// # Panics
    /// Panics if encounters unknown token
    #[must_use]
    pub fn scan(scanner: &mut Scanner) -> Option<Self> {
        let Some(char) = scanner.peek() else {
            return Some(Self::simple(scanner, Token::EOF))
        };

        match char {
            ':' => Some(Self::simple(scanner, Token::Colon)),
            '#' => Some(Self::simple(scanner, Token::Hash)),
            '-' => Some(Self::simple(scanner, Token::Minus)),
            '+' => Some(Self::simple(scanner, Token::Plus)),
            '.' => Some(Self::simple(scanner, Token::Dot)),
            ',' => Some(Self::simple(scanner, Token::Comma)),
            ';' => Self::comment(scanner),
            '\n' | '\r' => Self::linebreak(scanner),

            'a'..='z' | 'A'..='Z' | '_' => Self::literal(scanner, Literal::identifier),

            '0'..='9' => Self::literal(scanner, |s| Literal::number(s, 10)),
            '$' => {
                scanner.next();
                Self::literal(scanner, |s| Literal::number(s, 16))
            },

            '%' => {
                scanner.next();
                Self::literal(scanner, |s| Literal::number(s, 2))
            },

            ' ' | '\t' => {
                scanner.next(); // Ignore whitespace
                None
            }

            c => panic!("Encountered unknown token: {c}"),
        }
    }

    fn simple(scanner: &mut Scanner, token: Token) -> Self {
        let begin = scanner.location.clone();
        scanner.next();
        let end = scanner.location.clone();

        Self {
            token,
            span: Span::new(begin, end)
        }
    }


    fn comment(scanner: &mut Scanner) -> Option<Self> {
        scanner.skip_while(|&c| c != '\n' && c != '\r');
        Self::linebreak(scanner)
    }

    // This should be able to handle
    // Unix, classic Mac OS and Windows style endings
    fn linebreak(scanner: &mut Scanner) -> Option<Self> {
        let begin = scanner.location.clone();

        let newline = scanner.next()?;
        let peek = scanner.peek();
        let windows_lf = peek
            .filter(|&&c| c == '\n');

        if newline == '\r' && windows_lf.is_some() {
            scanner.next();
        }

        scanner.location.line += 1;
        scanner.location.column = 0;

        let end = scanner.location.clone();

        let token = Self {
            token: Token::LineBreak,
            span: Span::new(begin, end),
        };

        Some(token)
    }

    fn literal(scanner: &mut Scanner, f: impl FnOnce(&mut Scanner) -> Option<Literal>) -> Option<Self> {
        let begin = scanner.location.clone();
        let literal = f(scanner)?;
        let end = scanner.location.clone();

        Some(TokenWithSpan {
            token: Token::Literal(literal),
            span: Span::new(begin, end),
        })
    }
}


impl Span {
    fn new(begin: Location, end: Location) -> Self {
        Self {
            begin,
            end,
        }
    }
}
