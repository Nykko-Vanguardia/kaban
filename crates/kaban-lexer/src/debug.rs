use std::fmt::Debug;

use kaban_core::{UIndex, source::Source};

use crate::token::{Token, TokenKind};

pub struct TokenPrinter<'a> {
    source: Source<'a>,
    kind: TokenKind,
    start: UIndex,
    end: UIndex,
}

impl Token {
    pub fn to_debugger<'a>(&self, source: Source<'a>) -> TokenPrinter<'a> {
        TokenPrinter {
            source,
            kind: self.kind,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> Debug for TokenPrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = unsafe { str::from_utf8_unchecked(self.source.get(self.start, self.end)) };
        write!(f, "{:?}({:?})", self.kind, string)
    }
}
