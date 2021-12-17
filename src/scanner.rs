use crate::{error::SyntaxError, token::Token};

pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, SyntaxError> {
        Err(SyntaxError::new(3))
    }
}
