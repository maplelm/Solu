use super::core::Lexer;
use std::fmt;

pub struct LexerError {
    pub span: Span,
    pub msg: String,
}

impl LexerError {
    pub fn new(lex: &Lexer, msg: impl Into<String>) -> Self {
        Self {
            span: Span::new(lex),
            msg: msg.into(),
        }
    }
}

// Used to track where lexer is in file for error messaging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize, // Byte offset
    pub end: usize,   // Byte offset Exclusive
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(lex: &Lexer) -> Self {
        Self {
            start: lex.cursor,
            end: lex.cursor,
            line: lex.line,
            col: lex.cursor - lex.line_start,
        }
    }

    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
