use crate::models::Span;

use super::Token;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}
