use kaban_core::{UIndex, SourceSpan};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
}

impl Token {
    pub fn new(kind: TokenKind, start: UIndex, end: UIndex) -> Token {
        Token {kind, span: SourceSpan {start, end}}
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    //lits
    IntLit,
    FloatLit,
    StringLit,
    StringObjLit,  // for ``, automatically sugars to String.new()
    InterpolatedStringObjLit,  // for f`` automaticallu sugars to String.format()
    BoolLit,
    Char8Lit,
    Char16Lit,
    Char32Lit,

    //keywords
    Let,
    Mut,
    Const,
    // Alloc,
    // Kalloc,
    // Realloc,
    // Free,
    Struct,
    Interface,
    Impl,
    // Class,
    Pub,
    Constructor,
    Destructor,
    Disposer,
    Dispose,
    // New, //REMOVED FOR NOW, USE .new()
    // Destruct,
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
    Do,
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
    // Hash,
    Unsafe,
    ASM,

    //identifiers
    Identifier,

    //symbols
    Semicolon,
    Colon,
    ColonColon,
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
    Percent,
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
    PercentEquals, // %=
    Bang, // !
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
    GreaterGreaterGreater,
    ///@ symbol
    At,

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
    C8,
    C16,
    C32,
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
    DocComment,
    EOF,
    Invalid
}
