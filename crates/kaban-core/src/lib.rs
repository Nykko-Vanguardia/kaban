pub type SourceIndex = u32;

impl ToUsize for SourceIndex {
    #[inline(always)]
    fn usize(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Represents a byte range in the source text.
/// 
/// `start` is inclusive, `end` is exclusive.
/// To extract the text: `&source[span.start as usize..span.end as usize]`
/// 
/// # Example
/// ```
/// // source: "if x"
/// //          01234
/// // Token "if" → SourceSpan { start: 0, end: 2 }
/// // &source[0..2] == "if"
/// ```
pub struct SourceSpan {
    pub start: SourceIndex,
    pub end: SourceIndex,
}

pub trait ToUsize {
    fn usize(self) -> usize;
}
