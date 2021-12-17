use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, SyntaxError> {
        Err(SyntaxError { line: 4 })
    }
}

#[derive(Debug)]
pub struct Token;

#[derive(Debug)]
pub struct SyntaxError {
    line: u32,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "syntax error on line {}", self.line)
    }
}

impl Error for SyntaxError {}
