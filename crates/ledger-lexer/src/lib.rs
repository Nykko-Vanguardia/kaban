use std::io::Cursor;

pub enum Token<'a> {
    //lits
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    InterpolatedString(String),  // for ``
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
    Null,

    //Reserved
    Autofree, // debating if i should add this, autofree is only for class variables, will autofree
              // upon calling destructor()
    Singleton, // im debating if i should add this, singletons are just obj but are constructed
               // immediately with only one instance existing 
    Async, // maybe future async support??
    Await, // maybe future async support??
    Unsafe, //Might not need this 

    EOF,
    Invalid{
        error: LexError,
        line: usize,
        col: usize,
        cause: &'a str,
    },
}

//TODO add line and column of error
#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("Float literal must have a digit after the decimal point")]
    InvalidFloat,
    #[error("Unexpected character")]
    UnexpectedCharacter,
}

pub struct Lexer<'a> {
    source: &'a [u8],
    current: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str,) -> Self {
        Lexer { source: input.as_bytes(), current: 0, line: 1, col: 1 }
    }

    pub fn consume_next_token(&mut self) -> Token<'a> {
        while  Self::is_whitespace(self.source[self.current]){
            self.advance_current(); 
        }
        let current = self.peek_current();
        
        match current {
            b'\0' => Token::EOF,
            b'0'..=b'9' => self.handle_numbers(),
            x if Self::is_non_underscore_symbol(x) => self.handle_symbol(),
            _ => self.handle_keywords()
        }
    }

    fn handle_keywords(&mut self) -> Token<'a> {
        Token::Identifier("wazzup")
    }

    //TODO: Could just parse string of numbers to float itself
    fn handle_numbers(&mut self) -> Token<'a> {
        let mut number: i64 = 0;
        while Self::is_number(self.peek_current()) {
            let next_digit = (self.peek_current() - b'0') as i64;
            number = number * 10 + next_digit; 
            self.advance_current();
        };

        if self.peek_current() == b'.' {
            return self.handle_float(number);
        }

        Token::IntLiteral(number)
    }

    fn handle_float(&mut self, left_of_decimal: i64) -> Token<'a> {
        self.advance_current();
        if !Self::is_number(self.peek_current()) {
            return Token::Invalid { error: LexError::InvalidFloat, line: 1, col: 1, cause: "todo" }
        };

        let mut right_of_decimal = 0.0;
        let mut place = 10.0; //this is currently at the tenths place of the decimal, aka 0.X
        while Self::is_number(self.peek_current()) {
            let next_digit: f64 = (self.peek_current() - b'0') as f64 / place;
            right_of_decimal = right_of_decimal + next_digit;
            place = place * 10.0;
            self.advance_current();
        };

        let float = left_of_decimal as f64 + right_of_decimal;
        Token::FloatLiteral(float)
    }

    fn handle_symbol(&mut self) -> Token<'a> {
        let current = self.peek_current();
        match current {
            b'=' => match self.peek_next() {
                b'=' => Token::EqualEqual,
                b'>' => Token::FatArrow,
                _ => Token::Equals,
            },
            _ => Token::Invalid { error: LexError::UnexpectedCharacter, line: 0,  col: 0, cause: "todo" }
        }
    }

    fn advance_current(&mut self) {
        let current_byte = self.peek_current();
        self.current += Lexer::get_char_size(current_byte);
    }

    fn peek_current(&self) -> u8 {
        self.peek_till(0)
    }

    fn peek_next(&self) -> u8 {
        self.peek_till(1)
    }

    fn peek_till(&self, till: u8) -> u8 {
        if self.current >= self.source.len() {
            return b'\0';
        };

        self.source[self.current + till as usize]
    }

    fn is_whitespace(char_in_bytes: u8) -> bool {
        char_in_bytes == b'\n' || 
            char_in_bytes == b'\t' || 
            char_in_bytes == b'\r' ||
            char_in_bytes == b' '
    }

    fn is_number(char_in_bytes: u8) -> bool {
        matches!(char_in_bytes, b'0'..=b'9')
    }

    fn is_symbol(char_in_bytes: u8) -> bool {
        char_in_bytes.is_ascii() && (!char_in_bytes.is_ascii_alphanumeric())
    }

    fn is_non_underscore_symbol(char_in_bytes: u8) -> bool {
        Self::is_symbol(char_in_bytes) && char_in_bytes != b'_'
    }

    fn get_char_size(byte: u8) -> usize {
        if byte < 128 {
            1
        } else if byte & 0b1110_0000 == 0b1100_0000 {
            2
        } else if byte & 0b1111_0000 == 0b1110_0000 {
            3
        } else if byte & 0b1111_1000 == 0b1111_0000 {
            4
        } else {
            0 
        }
    }
}
