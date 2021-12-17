use std::{
    error::Error,
    fmt::{Display, Formatter},
};

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

impl SyntaxError {
    pub fn new(line: u32) -> Self {
        SyntaxError { line }
    }
}
