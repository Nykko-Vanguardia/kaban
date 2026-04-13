pub enum Token<'a> {
    //lits
    IntLit(i64),
    FloatLit(f64),
    StringLit(&'a str),
    StringObjLit(&'a str),  // for ``, automatically sugars to String.new()
    InterpolatedStringObjLit(&'a str),  // for f`` automaticallu sugars to String.format()
    BoolLit(bool),
    CharacterLit(char),

    //keywords
    Const,
    Let,
    Alloc,
    Buffer,
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
    Pass,
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
    Undefined,
    Garbage,

    //Reserved
    Autofree, // debating if i should add this, autofree is only for class variables, will autofree
              // upon calling destructor()
    Singleton, // im debating if i should add this, singletons are just obj but are constructed
               // immediately with only one instance existing 
    Async, // maybe future async support??
    Await, // maybe future async support??
    Unsafe, //Might not need this 

    DocComment(&'a str),
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
    #[error("Incomplete String")]
    IncompleteString,
    #[error("Invalid Unicode")]
    InvalidUnicode,
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

    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.consume_next_token();
            let is_eof = matches!(token, Token::EOF);
            tokens.push(token);
            if is_eof { break; }
        }

        tokens
    }

    pub fn consume_next_token(&mut self) -> Token<'a> {
        self.skip_whitespace_and_comments();
        let current = self.peek_current();
        
        match current {
            b'\0' => Token::EOF,
            b'0'..=b'9' => self.handle_numbers(),
            c if Self::is_non_underscore_symbol(c) => self.handle_symbol(),
            b'"' => self.handle_string(b'"', Token::StringLit),
            b'`' => self.handle_string(b'`',Token::StringObjLit),
            b'f' if self.peek_next() == b'`' => 
                self.handle_string(b'`',Token::InterpolatedStringObjLit),
            b'/' if self.is_doc_comment() => self.handle_doc_comment(),
            _ => self.handle_letters()
        }
    }

    fn handle_letters(&mut self) -> Token<'a> {
        let starting_index = self.current;
        while Self::is_keyword_or_identifier_char(self.peek_current()) {
            self.advance_current();
        };
        let ending_index = self.current;
        let keyword_or_identifier = match str::from_utf8(&self.source[starting_index..ending_index]) {
            Ok(s) => s,
            Err(_) => return Self::get_invalid(&self, LexError::InvalidUnicode),
        };

        match keyword_or_identifier {
            "let" => Token::Let,
            _ => Token::Identifier(keyword_or_identifier),
        }
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

        Token::IntLit(number)
    }

    fn handle_float(&mut self, left_of_decimal: i64) -> Token<'a> {
        self.advance_current();
        if !Self::is_number(self.peek_current()) {
            return self.get_invalid(LexError::InvalidFloat)
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
        Token::FloatLit(float)
    }

    fn handle_symbol(&mut self) -> Token<'a> {
        let current = self.peek_current();
        match current {
            b'=' => match self.peek_next() {
                b'=' => Token::EqualEqual,
                b'>' => Token::FatArrow,
                _ => Token::Equals,
            },
            _ => self.handle_letters(),
        }
    }

    fn handle_string(&mut self, terminator: u8, token_constructor: fn(&'a str) -> Token<'a>) -> Token<'a> {
        if self.peek_current() == b'f' {self.advance_current();};
        self.advance_current();
        let starting_char_index = self.current;
        while self.peek_current() != terminator {
            if self.peek_current() == b'\0' { return self.get_invalid(LexError::IncompleteString) };

            self.advance_current();
        };

        let end_quotes_index = self.current;
        self.advance_current(); //consume end quotes
        let char_slice = &self.source[starting_char_index..end_quotes_index];
        match str::from_utf8(char_slice) {
            Ok(s) => token_constructor(s),
            Err(_) => self.get_invalid(LexError::InvalidUnicode),
        }
    }

    fn handle_doc_comment(&mut self) -> Token<'a> {
        for _ in 0..3 { self.advance_current(); }
        let start_comment_index = self.current;

        while !(self.peek_current() == b'*' && self.peek_next() == b'/') 
          && self.peek_current() != b'\0' {
            self.advance_current();
        }
        let last_comment_index = self.current;
        self.advance_current();
        self.advance_current();
        let comment = match  str::from_utf8(&self.source[start_comment_index..last_comment_index]) {
            Ok(s) => s,
            Err(_) => return self.get_invalid(LexError::InvalidUnicode),
        };
        Token::DocComment(comment)
    }

    fn advance_current(&mut self) {
        let current_byte = self.peek_current();
        if current_byte == b'\0' {
            return;
        }

        if current_byte == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        self.current += Lexer::get_char_size(current_byte);
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek_current() {
                c if c.is_ascii_whitespace() => self.advance_current(),
                b'/' if self.peek_next() == b'/' => {
                    while self.peek_current() != b'\n' && self.peek_current() != b'\0'{
                        self.advance_current();
                    }
                }
                b'/' if self.peek_next() == b'*' =>  {
                    if self.is_doc_comment() { break };
                    self.advance_current();
                    self.advance_current();

                    while !(self.peek_current() == b'*' && self.peek_next() == b'/') 
                        && self.peek_current() != b'\0' {self.advance_current();}
                    self.advance_current();
                    self.advance_current();
                }
                _ => break,
            }
        }
    }

    fn peek_current(&self) -> u8 {
        self.peek_till(0)
    }

    fn peek_next(&self) -> u8 {
        self.peek_till(1)
    }

    fn peek_till(&self, till: usize) -> u8 {
        if self.current + till >= self.source.len() {
            return b'\0';
        };

        self.source[self.current + till as usize]
    }

    // fn is_whitespace(char_in_bytes: u8) -> bool {
    //     char_in_bytes == b'\n' || 
    //         char_in_bytes == b'\t' || 
    //         char_in_bytes == b'\r' ||
    //         char_in_bytes == b' '
    // }

    fn is_number(char_in_bytes: u8) -> bool {
        matches!(char_in_bytes, b'0'..=b'9')
    }

    fn is_symbol(char_in_bytes: u8) -> bool {
        char_in_bytes.is_ascii() && (!char_in_bytes.is_ascii_alphanumeric())
    }

    fn is_non_underscore_symbol(char_in_bytes: u8) -> bool {
        Self::is_symbol(char_in_bytes) && char_in_bytes != b'_'
    }

    fn is_keyword_or_identifier_char(char_in_bytes: u8) -> bool {
        let is_unicode = char_in_bytes > 128;
        char_in_bytes.is_ascii_alphanumeric() || char_in_bytes == b'_' || is_unicode
    }

    fn is_doc_comment(&self) -> bool {
        self.peek_current() == b'/' &&
            self.peek_next() == b'*' &&
            self.peek_till(2) == b'*' &&
            self.peek_till(3) != b'/'
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

    fn get_invalid(&self, lex_error: LexError) -> Token<'a> {
        Token::Invalid { error: lex_error, line: self.line, col: self.col, cause: "todo" }
    }
}
