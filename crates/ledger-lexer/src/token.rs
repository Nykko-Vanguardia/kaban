use crate::LexError;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    //lits
    IntLit(&'a str),
    FloatLit(&'a str),
    StringLit(&'a str),
    StringObjLit(&'a str),  // for ``, automatically sugars to String.new()
    InterpolatedStringObjLit(&'a str),  // for f`` automaticallu sugars to String.format()
    BoolLit(bool),
    Char8Lit(u8),
    Char16Lit(&'a [u8]),
    Char32Lit(&'a [u8]),

    //keywords
    Let,
    Mut,
    Const,
    Alloc,
    Kalloc,
    Realloc,
    Free,
    Struct,
    Interface,
    Impl,
    Class,
    Pub,
    Constructor,
    Destructor,
    Disposer,
    Dispose,
    New,
    Destruct,
    Self_,
    Shape,
    Namespace,
    Result,
    Ok,
    Err,
    Exit,
    If,
    Else,
    Match,
    For,
    In,
    Break,
    Continue,
    Pass,
    While,
    Return,
    Func,
    Union,
    Enum,
    Is,
    To,
    Import,
    Type,
    As,
    Band,
    Bor,
    Bxor,
    Bnot,
    Comptime,
    Write,
    Hash,
    Unsafe,
    ASM,

    //identifiers
    Identifier(&'a str),

    //symbols
    Semicolon,
    Colon,
    Comma,
    Equals,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket, // [
    RightBracket,
    Star, // * for heap pointers
    Caret, // ^ for deref
    Ampersand, // & for borrows
    AmpersandMut, //&mut
    Pipe, // | for union types
    FatArrow, // => Im still not sure between these two
    SkinnyArrow, // -> Im still not sure between these two
    Plus,
    Minus,
    PlusPlus, //++
    MinusMinus, //--
    Slash,
    DotDot, // ..
    DotDotDot, // ...
    DotDotEquals, // ..=
    PlusEquals, // += 
    MinusEquals, // -=
    StarEquals, // *=
    SlashEquals, // /=
    Bang, // !
    BangDot,
    Question,
    QuestionQuestion,
    QuestionQuestionDot,
    Less,
    Greater,
    LessEqual,
    GreaterEqual, // >=
    EqualEqual, // ==
    BangEqual, // !=
    And, // &&
    Or, // ||
    Dot, // . for field access
    LessLess, // << bitwise
    GreaterGreater, // >> bit wise

    //types
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    U8, 
    U16, 
    U32, 
    U64,
    USize,
    Char8,
    Char16,
    Char32,
    Bool,
    Void,
    Undefined,
    Garbage,

    //Reserved
    Autofree, // debating if i should add this, autofree is only for class variables, will autofree
              // upon calling destructor()
    Async,
    Await,
    Heap, //Might replace alloc
    Raw,
    LeftArrow, //Maybe ill use this
    Where,
    Defer,
    Nil,

    //Special
    DocComment(&'a str),
    EOF,
    Invalid{
        error: LexError,
        line: usize,
        col: usize,
        cause: &'a str,
    },
}
