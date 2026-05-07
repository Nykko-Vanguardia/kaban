use std::fmt::Debug;

use kaban_core::{SourceSpan, source::Source};

use crate::{Token, token::TokenKind};

pub struct TokenPrinter<'a> {
    source: Source<'a>,
    kind: TokenKind,
    span: SourceSpan,
}

impl Token {
    pub fn to_debugger<'a>(&self, source: Source<'a>) -> TokenPrinter<'a> {
        TokenPrinter {
            source,
            kind: self.kind,
            span: self.span,
        }
    }
}

impl<'a> Debug for TokenPrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = unsafe {
            str::from_utf8_unchecked(self.source.get(self.span))
        };
        write!(f, "{:?}({:?})", self.kind, string)
    }
}
