pub enum Token {
    //lits
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    CharacterLiteral(char),

    //keywords
    Const,
    Let,
    Alloc,
    Stack,
    Free,
    Struct,
    Interface,
    Implements,   
    Obj,
    Error,
    Public,
    Private,
    If,
    Else,
    Match,
    For,
    In,
    Break,
    Continue,
    While,
    Return,
    Func,
    Union,
    Import,
    Type,
    Is,
    As,
    Band, // band
    Bor, // bor
    Bxor, // bxor
    Bnot, // bnot

    //identifiers
    Identifier(String),

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
    FatArrow, // =>
    Plus,
    Minus,
    PlusPlus, //++
    MinusMinus, //--
    Slash,
    DotDot, // ..
    DotDotEquals, // ..=
    PlusEquals, // += 
    MinusEquals, // -=
    StarEquals, // *=
    SlashEquals, // /=
    Bang, // !
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
    StringType,
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
    Char,
    Bool,
    Void,

    //Reserved
    Autofree, // debating if i should add this, autofree is only for class variables, will autofree
              // upon calling destructor()
    Singleton, // im debating if i should add this, singletons are just obj but are constructed
               // immediately with only one instance existing 
    Async, // maybe future async support??
    Await, // maybe future async support??
    Unsafe, //Might not need this 

    EOF,
}

pub struct Lexer {
    pub source: String,
    pub current: usize,
    pub line: usize,
    pub col: usize,
}

impl Lexer {

}
