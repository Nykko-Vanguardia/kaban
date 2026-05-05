use kaban_core::SourceIndex;
use kaban_core::ToUsize;

use crate::Token;
use crate::LexError;
use crate::token::TokenKind;

pub struct Lexer<'a> {
    source: &'a [u8],
    current: SourceIndex,
    pub errors: Vec<LexError>,
    line: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str,) -> Self {
        Lexer { source: input.as_bytes(), errors: Vec::new(), current: 0, line: 1, col: 1 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.consume_next_token();
            let is_eof = matches!(token.kind, TokenKind::EOF);
            tokens.push(token);
            if is_eof { break; }
        }

        tokens
    }

    pub fn consume_next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        let current = self.peek_current();
        let start = self.current;
        
        let kind = match current {
            b'\0' => TokenKind::EOF,
            b'0'..=b'9' => self.handle_numbers(),
            b'/' if self.is_doc_comment() => self.handle_doc_comment(),
            b'"' => self.handle_string(b'"', TokenKind::StringLit),
            b'`' => self.handle_string(b'`', TokenKind::StringObjLit),
            b'f' if self.peek_next() == b'`' => 
                self.handle_string(b'`',TokenKind::InterpolatedStringObjLit),
            b'\'' => self.handle_char_lit(),
            c if Self::is_non_underscore_symbol(c) => self.handle_symbol(),
            _ => self.handle_letters()
        };
        let end = self.current;
        Token::new(kind, start, end)
    }

    fn handle_letters(&mut self) -> TokenKind {
        let starting_index = self.current;
        while Self::is_keyword_or_identifier_char(self.peek_current()) {
            self.advance_current();
        };
        let ending_index = self.current;
        let keyword_or_identifier = match str::from_utf8(&self.source[starting_index as usize..ending_index as usize]) {
            Ok(s) => s,
            Err(_) => return self.error_recovery(LexError::InvalidUnicode),
        };

        match keyword_or_identifier {
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "const" => TokenKind::Const,
            // "alloc" => TokenKind::Alloc,
            // "kalloc" => TokenKind::Kalloc, //might not need this
            // "realloc" => TokenKind::Realloc,
            // "free" => TokenKind::Free,
            "struct" => TokenKind::Struct,
            "interface" => TokenKind::Interface,
            "impl" => TokenKind::Impl,
            "class" => TokenKind::Class,
            "pub" => TokenKind::Pub,
            "constructor" => TokenKind::Constructor,
            "destructor" => TokenKind::Destructor,
            "new" => TokenKind::New,
            "destruct" => TokenKind::Destruct,
            "disposer" => TokenKind::Disposer,
            "dispose" => TokenKind::Dispose,
            "self" => TokenKind::Self_,
            // "shape" => TokenKind::Shape, //This should be an identifier, we use it in impl blocks
            // based on context. But shape is a keyword, only in impl blcoks
            "namespace" => TokenKind::Namespace,
            "result" => TokenKind::Result,
            "ok" => TokenKind::Ok,
            "err" => TokenKind::Err,
            "exit" => TokenKind::Exit,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "pass" => TokenKind::Pass,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "func" => TokenKind::Func,
            "union" => TokenKind::Union,
            "enum" => TokenKind::Enum,
            "is" => TokenKind::Is,
            "to" => TokenKind::To,
            "import" => TokenKind::Import,
            "type" => TokenKind::Type,
            "as" => TokenKind::As,
            "band" => TokenKind::Band,
            "bor" => TokenKind::Bor,
            "bxor" => TokenKind::Bxor,
            "bnot" => TokenKind::Bnot,
            "comptime" => TokenKind::Comptime,
            "write" => TokenKind::Write,
            "unsafe" => TokenKind::Unsafe,
            //Types
            "i8" => TokenKind::I8,
            "i16" => TokenKind::I16,
            "i32" => TokenKind::I32,
            "i64" => TokenKind::I64,
            "f32" => TokenKind::F32,
            "f64" => TokenKind::F64,
            "u8" => TokenKind::U8, 
            "u16" => TokenKind::U16, 
            "u32" => TokenKind::U32, 
            "u64" => TokenKind::U64,
            "usize" => TokenKind::USize,
            "c8" => TokenKind::C8,
            "c16" => TokenKind::C16,
            "c32" => TokenKind::C32,
            "bool" => TokenKind::Bool,
            "void" => TokenKind::Void,
            "undefined" => TokenKind::Undefined,
            "garbage" => TokenKind::Garbage,
            "asm" => TokenKind::ASM,
            //Reserved
            "autofree" => TokenKind::Autofree,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "heap" => TokenKind::Heap, //Might replace alloc
            "raw" => TokenKind::Raw,
            "where" => TokenKind::Where,
            "defer" => TokenKind::Defer,
            "nil" => TokenKind::Nil,
            _ => TokenKind::Identifier,
        }
    }

    fn handle_symbol(&mut self) -> TokenKind {
        let current = self.peek_current();
        match current {
            b'=' => {
                if self.match_and_consume("==") {
                    TokenKind::EqualEqual
                } else if self.match_and_consume("=>") {
                    TokenKind::FatArrow
                } else {
                    self.advance_current();
                    TokenKind::Equals
                }
            },
            b'!' => {
                if self.match_and_consume("!=") {
                    TokenKind::BangEqual
                } else if self.match_and_consume("!.") {
                    TokenKind::BangDot
                } else {
                    self.advance_current();
                    TokenKind::Bang
                }
            },
            b'?' => {
                if self.match_and_consume("?.") {
                    TokenKind::QuestionDot
                } else if self.match_and_consume("??.") {
                    TokenKind::QuestionQuestionDot
                } else if self.match_and_consume("??") {
                    TokenKind::QuestionQuestion
                } else {
                    self.advance_current();
                    TokenKind::Question
                }
            },
            b'<' => {
                if self.match_and_consume("<<") {
                    TokenKind::LessLess
                } else if self.match_and_consume("<=") {
                    TokenKind::LessEqual
                } else if self.match_and_consume("<-") {
                    TokenKind::LeftArrow
                } else {
                    self.advance_current();
                    TokenKind::Less
                }
            },
            b'>' => {
                if self.match_and_consume(">>") {
                    TokenKind::GreaterGreater
                } else if self.match_and_consume(">>>") {
                    TokenKind::GreaterGreaterGreater
                } else if self.match_and_consume(">=") {
                    TokenKind::GreaterEqual
                } else {
                    self.advance_current();
                    TokenKind::Greater
                }
            },
            b'&' => {
                if self.match_and_consume("&&") {
                    TokenKind::And
                } else if self.match_and_consume("&mut") {
                    TokenKind::AmpersandMut
                } else {
                    self.advance_current();
                    TokenKind::Ampersand
                }
            },
            b'|' => {
                if self.match_and_consume("||") {
                    TokenKind::Or
                } else {
                    self.advance_current();
                    TokenKind::Pipe
                }
            },
            b'+' => {
                if self.match_and_consume("++") {
                    TokenKind::PlusPlus
                } else if self.match_and_consume("+=") {
                    TokenKind::PlusEquals
                } else {
                    self.advance_current();
                    TokenKind::Plus
                }
            },
            b'-' => {
                if self.match_and_consume("--") {
                    TokenKind::MinusMinus
                } else if self.match_and_consume("-=") {
                    TokenKind::MinusEquals
                } else if self.match_and_consume("->") {
                    TokenKind::SkinnyArrow
                } else {
                    self.advance_current();
                    TokenKind::Minus
                }
            },
            b'*' => {
                if self.match_and_consume("*=") {
                    TokenKind::StarEquals
                } else {
                    self.advance_current();
                    TokenKind::Star
                }
            },
            b'/' => {
                if self.match_and_consume("/=") {
                    TokenKind::SlashEquals
                } else {
                    self.advance_current();
                    TokenKind::Slash
                }
            },
            b'.' => {
                if self.match_and_consume("...") {
                    TokenKind::DotDotDot
                } else if self.match_and_consume("..=") {
                    TokenKind::DotDotEquals
                } else if self.match_and_consume("..") {
                    TokenKind::DotDot
                } else {
                    self.advance_current();
                    TokenKind::Dot
                }
            },
            b';' => { self.advance_current(); TokenKind::Semicolon },
            b':' => { self.advance_current(); TokenKind::Colon },
            b',' => { self.advance_current(); TokenKind::Comma },
            b'{' => { self.advance_current(); TokenKind::LeftBrace },
            b'}' => { self.advance_current(); TokenKind::RightBrace },
            b'(' => { self.advance_current(); TokenKind::LeftParen },
            b')' => { self.advance_current(); TokenKind::RightParen },
            b'[' => { self.advance_current(); TokenKind::LeftBracket }
            b']' => { self.advance_current(); TokenKind::RightBracket },
            b'^' => { self.advance_current(); TokenKind::Caret },
            b'#' => { self.advance_current(); TokenKind::Hash },
            b'%' => { self.advance_current(); TokenKind::Percent },
            _ => self.error_recovery(LexError::UnexpectedCharacter),
        }
    }

    fn handle_numbers(&mut self) -> TokenKind {
        if self.peek_current() == b'0' {
            match self.peek_next() {
                b'b' | b'B' => return self.handle_bin(),
                b'x' | b'X' => return self.handle_hex(),
                b'o' | b'O' => return self.handle_oct(),
                _ => {}
            };
        };

        while self.peek_current().is_ascii_digit() { self.advance_current(); };

        let mut float_flag = self.peek_current() == b'.' && self.peek_next().is_ascii_digit();
        if float_flag {
            self.advance_current();
            while self.peek_current().is_ascii_digit() { self.advance_current();}
        }

        if self.peek_current() == b'e' || self.peek_current() == b'E' {
            let has_sign = self.peek_next() == b'+' || self.peek_next() == b'-';
            let next_digit = if has_sign { self.peek_till(2) } else { self.peek_next() };

            if next_digit.is_ascii_digit() {
                float_flag = true;
                self.advance_current(); //consume e
                if has_sign { self.advance_current(); } //consume sign
                while self.peek_current().is_ascii_digit() {
                    self.advance_current();
                }
            }
        }

        if !float_flag {TokenKind::IntLit} else {TokenKind::FloatLit}
    }

    fn handle_hex(&mut self) -> TokenKind {
        self.advance_current();
        self.advance_current();
        while self.peek_current().is_ascii_hexdigit() {
            self.advance_current();
        }
        TokenKind::IntLit
    }

    fn handle_bin(&mut self) -> TokenKind {
        self.advance_current();
        self.advance_current();
        while self.peek_current() == b'0' || self.peek_current() == b'1' {
            self.advance_current();
        }

        TokenKind::IntLit
    }

    fn handle_oct(&mut self) -> TokenKind {
        self.advance_current();
        self.advance_current();
        while matches!(self.peek_current(), b'0'..=b'7'){
            self.advance_current();
        }

        TokenKind::IntLit
    }

    fn handle_string(&mut self, terminator: u8, token_kind: TokenKind) -> TokenKind {
        if self.peek_current() == b'f' {self.advance_current();};
        self.advance_current();
        while self.peek_current() != terminator {
            if self.peek_current() == b'\0' { return self.error_recovery(LexError::IncompleteString) };
            if self.peek_current() == b'\\' {
                self.advance_current();
                self.advance_current();
                continue;
            }

            self.advance_current();
        };
        self.advance_current(); //consume end quotes
        token_kind
    }

    fn handle_char_lit(&mut self) -> TokenKind {
        self.advance_current();

        let char = self.peek_current();
        let char = if char == b'\\' {
            self.advance_current();
            let escape_char = self.peek_current();
            match escape_char {
                b'n' => b'\n',
                b'r' => b'\r',
                b't' => b'\t',
                b'\\' => b'\\',
                b'\'' => b'\'',
                _ => escape_char,
            }
        } else { char };

        self.advance_current();
        if self.peek_current() != b'\'' { return self.error_recovery(LexError::InvalidCharLiteral)};
        self.advance_current();

        match Self::get_char_size(char) {
            1 => TokenKind::Char8Lit,
            2 => TokenKind::Char16Lit,
            _ => TokenKind::Char32Lit,
        }
    }

    fn handle_doc_comment(&mut self) -> TokenKind {
        for _ in 0..3 { self.advance_current(); }

        while !(self.peek_current() == b'*' && self.peek_next() == b'/') 
            && self.peek_current() != b'\0' {
                self.advance_current();
            }
        self.advance_current();
        self.advance_current();

        TokenKind::DocComment
    }
}

//Helpers
impl<'a> Lexer<'a> {
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

    #[inline(always)]
    fn peek_current(&self) -> u8 {
        self.peek_till(0)
    }

    #[inline(always)]
    fn peek_next(&self) -> u8 {
        self.peek_till(1)
    }

    #[inline(always)]
    pub fn peek_till(&self, offset: usize) -> u8 {
        self.source.get(self.current.usize() + offset).copied().unwrap_or(b'\0')
    }

    fn matches_current(&self, pattern: &str) -> bool {
        let bytes = pattern.as_bytes();
        let end = self.current.usize() + bytes.len();
        if end > self.source.len() {
            return false;
        }

        &self.source[self.current.usize()..end] == bytes
    }

    fn match_and_consume(&mut self, pattern: &str) -> bool {        
        if !self.matches_current(pattern) { return false; };

        for _ in 0..pattern.as_bytes().len() {
            self.advance_current();
        };

        true
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

    pub fn get_char_size(byte: u8) -> SourceIndex {
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

    fn error_recovery(&mut self, lex_error: LexError) -> TokenKind {
        self.errors.push(lex_error);
        self.advance_current();
        TokenKind::Invalid
    }
}
