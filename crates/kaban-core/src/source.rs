use crate::{UIndex, SourceSpan, ToUIndex, ToUsize};

#[derive(Clone, Copy)]
pub struct Source<'a> {
    source: &'a [u8]
}

impl<'a> Source<'a> {
    pub fn new(source: &'a str) -> Source<'a> {
        Source { source: source.as_bytes() }
    }

    #[inline(always)]
    pub fn get(&self, span: SourceSpan) -> &'a [u8] {
        &self.source[span.start.usize()..span.end.usize()]
    }

    #[inline(always)]
    pub fn get_start_end(&self, start: UIndex, end: UIndex) -> &'a[u8] {
        &self.source[start.usize()..end.usize()]
    }

    #[inline(always)]
    pub fn char(&self, index: UIndex) -> u8 {
        self.source.get(index.usize()).copied().unwrap_or(b'\0')
    }

    #[inline(always)]
    pub fn len(&self) -> UIndex {
        self.source.len().uindex()
    }

    #[inline(always)]
    pub fn matches(&self, span: SourceSpan, matches: &str) -> bool {
        self.get(span) == matches.as_bytes()
    }

    #[inline(always)]
    pub fn as_str(&self, span: SourceSpan) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.get(span))
        }
    }

    #[inline(always)]
    pub fn as_str_start_end(&self, start: UIndex, end: UIndex) -> &str {
        self.as_str(SourceSpan { start, end })
    }

    #[inline(always)]
    pub fn get_source_as_str(&self) -> &str {
        self.as_str_start_end(0, self.source.len().uindex())
    }
}

impl<'a> IsSource<'a> for &str {
    fn to_source(&'a self) -> Source<'a> {
        Source { source: self.as_bytes() }
    }
}

pub trait IsSource<'a> {
    fn to_source(&'a self) -> Source<'a>;
}
