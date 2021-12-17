use crate::token_kind::TokenKind;

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    lexeme: &'a str,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, lexeme: &'a str, line: u32) -> Self {
        Token { kind, lexeme, line }
    }
}
