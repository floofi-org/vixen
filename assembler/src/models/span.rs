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

impl Span {
    #[must_use]
    pub fn new(begin: Location, end: Location) -> Self {
        Self {
            begin,
            end,
        }
    }
}
