pub mod source;
pub type UIndex = u32;

impl ToUsize for UIndex {
    #[inline(always)]
    fn usize(self) -> usize {
        self as usize
    }
}

impl ToUIndex for usize {
    #[inline(always)]
    fn uindex(self) -> UIndex {
        self as UIndex
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
    pub start: UIndex,
    pub end: UIndex,
}

pub trait ToUsize {
    fn usize(self) -> usize;
}

pub trait ToUIndex {
    fn uindex(self) -> UIndex;
}

impl ToUIndex for bool {
    ///RETURNS 1 IF TRUE, 0 IF FALSE
    #[inline(always)]
    fn uindex(self) -> UIndex {
        if self == true {
            1
        } else {
            0
        }
    }
}

impl ToBool for UIndex {
    ///Converts 1 to true
    ///Converts 0 to false
    #[inline(always)]
    fn bool(self) -> bool {
        debug_assert!(self == 1 || self == 0, "tried to convert a non 0 or 1 uindex to a bool");
        self == 1
    }
}

pub trait ToBool {
    fn bool(self) -> bool;
}
