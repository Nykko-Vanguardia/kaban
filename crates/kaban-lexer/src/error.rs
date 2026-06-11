use std::fmt::{Debug, Display};

use kaban_core::{UIndex, compiler_error::CompilerError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    InvalidFloat,
    UnexpectedCharacter,
    IncompleteString,
    InvalidUnicode,
    InvalidCharLiteral,
}

pub struct LexError {
    pub kind: LexErrorKind,
    pub position: UIndex,
}

impl CompilerError for LexError {
    fn message(&self) -> String {
        let kind_message = match self.kind {
            LexErrorKind::InvalidFloat => "Float literal must have a digit after the decimal point",
            LexErrorKind::UnexpectedCharacter => "Unexpected character",
            LexErrorKind::IncompleteString => "Incomplete String",
            LexErrorKind::InvalidUnicode => "Invalid Unicode",
            LexErrorKind::InvalidCharLiteral => "Char Literals must contain one character",
        };

        format!("LexError at {}: {kind_message}", self.position)
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
