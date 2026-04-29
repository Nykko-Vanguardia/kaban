use crate::Token;
use crate::LexError;

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
            b'/' if self.is_doc_comment() => self.handle_doc_comment(),
            b'"' => self.handle_string(b'"', Token::StringLit),
            b'`' => self.handle_string(b'`',Token::StringObjLit),
            b'f' if self.peek_next() == b'`' => 
                self.handle_string(b'`',Token::InterpolatedStringObjLit),
            b'\'' => self.handle_char_lit(),
            c if Self::is_non_underscore_symbol(c) => self.handle_symbol(),
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
            "mut" => Token::Mut,
            "const" => Token::Const,
            // "alloc" => Token::Alloc,
            // "kalloc" => Token::Kalloc, //might not need this
            // "realloc" => Token::Realloc,
            // "free" => Token::Free,
            "struct" => Token::Struct,
            "interface" => Token::Interface,
            "impl" => Token::Impl,
            "class" => Token::Class,
            "pub" => Token::Pub,
            "constructor" => Token::Constructor,
            "destructor" => Token::Destructor,
            "new" => Token::New,
            "destruct" => Token::Destruct,
            "disposer" => Token::Disposer,
            "dispose" => Token::Dispose,
            "self" => Token::Self_,
            // "shape" => Token::Shape, //This should be an identifier, we use it in impl blocks
            // based on context. But shape is a keyword, only in impl blcoks
            "namespace" => Token::Namespace,
            "result" => Token::Result,
            "ok" => Token::Ok,
            "err" => Token::Err,
            "exit" => Token::Exit,
            "if" => Token::If,
            "else" => Token::Else,
            "match" => Token::Match,
            "for" => Token::For,
            "in" => Token::In,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "pass" => Token::Pass,
            "while" => Token::While,
            "return" => Token::Return,
            "func" => Token::Func,
            "union" => Token::Union,
            "enum" => Token::Enum,
            "is" => Token::Is,
            "to" => Token::To,
            "import" => Token::Import,
            "type" => Token::Type,
            "as" => Token::As,
            "band" => Token::Band,
            "bor" => Token::Bor,
            "bxor" => Token::Bxor,
            "bnot" => Token::Bnot,
            "comptime" => Token::Comptime,
            "write" => Token::Write,
            "unsafe" => Token::Unsafe,
            //Types
            "i8" => Token::I8,
            "i16" => Token::I16,
            "i32" => Token::I32,
            "i64" => Token::I64,
            "f32" => Token::F32,
            "f64" => Token::F64,
            "u8" => Token::U8, 
            "u16" => Token::U16, 
            "u32" => Token::U32, 
            "u64" => Token::U64,
            "usize" => Token::USize,
            "char8" => Token::Char8,
            "char16" => Token::Char16,
            "char32" => Token::Char32,
            "bool" => Token::Bool,
            "void" => Token::Void,
            "undefined" => Token::Undefined,
            "garbage" => Token::Garbage,
            "asm" => Token::ASM,
            //Reserved
            "autofree" => Token::Autofree,
            "async" => Token::Async,
            "await" => Token::Await,
            "heap" => Token::Heap, //Might replace alloc
            "raw" => Token::Raw,
            "where" => Token::Where,
            "defer" => Token::Defer,
            "nil" => Token::Nil,
            _ => Token::Identifier(keyword_or_identifier),
        }
    }

    fn handle_symbol(&mut self) -> Token<'a> {
        let current = self.peek_current();
        match current {
            b'=' => {
                if self.match_and_consume("==") {
                    Token::EqualEqual
                } else if self.match_and_consume("=>") {
                    Token::FatArrow
                } else {
                    self.advance_current();
                    Token::Equals
                }
            },
            b'!' => {
                if self.match_and_consume("!=") {
                    Token::BangEqual
                } else if self.match_and_consume("!.") {
                    Token::BangDot
                } else {
                    self.advance_current();
                    Token::Bang
                }
            },
            b'?' => {
                if self.match_and_consume("?.") {
                    Token::QuestionDot
                } else if self.match_and_consume("??.") {
                    Token::QuestionQuestionDot
                } else if self.match_and_consume("??") {
                    Token::QuestionQuestion
                } else {
                    self.advance_current();
                    Token::Question
                }
            },
            b'<' => {
                if self.match_and_consume("<<") {
                    Token::LessLess
                } else if self.match_and_consume("<=") {
                    Token::LessEqual
                } else if self.match_and_consume("<-") {
                    Token::LeftArrow
                } else {
                    self.advance_current();
                    Token::Less
                }
            },
            b'>' => {
                if self.match_and_consume(">>") {
                    Token::GreaterGreater
                } else if self.match_and_consume(">>>") {
                    Token::GreaterGreaterGreater
                } else if self.match_and_consume(">=") {
                    Token::GreaterEqual
                } else {
                    self.advance_current();
                    Token::Greater
                }
            },
            b'&' => {
                if self.match_and_consume("&&") {
                    Token::And
                } else if self.match_and_consume("&mut") {
                    Token::AmpersandMut
                } else {
                    self.advance_current();
                    Token::Ampersand
                }
            },
            b'|' => {
                if self.match_and_consume("||") {
                    Token::Or
                } else {
                    self.advance_current();
                    Token::Pipe
                }
            },
            b'+' => {
                if self.match_and_consume("++") {
                    Token::PlusPlus
                } else if self.match_and_consume("+=") {
                    Token::PlusEquals
                } else {
                    self.advance_current();
                    Token::Plus
                }
            },
            b'-' => {
                if self.match_and_consume("--") {
                    Token::MinusMinus
                } else if self.match_and_consume("-=") {
                    Token::MinusEquals
                } else if self.match_and_consume("->") {
                    Token::SkinnyArrow
                } else {
                    self.advance_current();
                    Token::Minus
                }
            },
            b'*' => {
                if self.match_and_consume("*=") {
                    Token::StarEquals
                } else {
                    self.advance_current();
                    Token::Star
                }
            },
            b'/' => {
                if self.match_and_consume("/=") {
                    Token::SlashEquals
                } else {
                    self.advance_current();
                    Token::Slash
                }
            },
            b'.' => {
                if self.match_and_consume("...") {
                    Token::DotDotDot
                } else if self.match_and_consume("..=") {
                    Token::DotDotEquals
                } else if self.match_and_consume("..") {
                    Token::DotDot
                } else {
                    self.advance_current();
                    Token::Dot
                }
            },
            b';' => { self.advance_current(); Token::Semicolon },
            b':' => { self.advance_current(); Token::Colon },
            b',' => { self.advance_current(); Token::Comma },
            b'{' => { self.advance_current(); Token::LeftBrace },
            b'}' => { self.advance_current(); Token::RightBrace },
            b'(' => { self.advance_current(); Token::LeftParen },
            b')' => { self.advance_current(); Token::RightParen },
            b'[' => { self.advance_current(); Token::LeftBracket },
            b']' => { self.advance_current(); Token::RightBracket },
            b'^' => { self.advance_current(); Token::Caret },
            b'#' => { self.advance_current(); Token::Hash },
            b'%' => { self.advance_current(); Token::Percent },
            _ => self.get_invalid(LexError::UnexpectedCharacter),
        }
    }

    fn handle_numbers(&mut self) -> Token<'a> {
        let start = self.current;

        if self.peek_current() == b'0' {
            match self.peek_next() {
                b'b' | b'B' => return self.handle_bin(start),
                b'x' | b'X' => return self.handle_hex(start),
                b'o' | b'O' => return self.handle_oct(start),
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

        let end = self.current;
        let num = str::from_utf8(&self.source[start..end]).unwrap();
        if !float_flag {Token::IntLit(num)} else {Token::FloatLit(num)}
    }

    fn handle_hex(&mut self, start: usize) -> Token<'a> {
        self.advance_current();
        self.advance_current();
        while self.peek_current().is_ascii_hexdigit() {
            self.advance_current();
        }
        let end = self.current;
        Token::IntLit(str::from_utf8(&self.source[start..end]).unwrap())
    }

    fn handle_bin(&mut self, start: usize) -> Token<'a> {
        self.advance_current();
        self.advance_current();
        while self.peek_current() == b'0' || self.peek_current() == b'1' {
            self.advance_current();
        }

        let end = self.current;
        Token::IntLit(str::from_utf8(&self.source[start..end]).unwrap())
    }

    fn handle_oct(&mut self, start: usize) -> Token<'a> {
        self.advance_current();
        self.advance_current();
        while matches!(self.peek_current(), b'0'..=b'7'){
            self.advance_current();
        }

        let end = self.current;
        Token::IntLit(str::from_utf8(&self.source[start..end]).unwrap())
    }

    fn handle_string(&mut self, terminator: u8, token_constructor: fn(&'a str) -> Token<'a>) -> Token<'a> {
        if self.peek_current() == b'f' {self.advance_current();};
        self.advance_current();
        let starting_char_index = self.current;
        while self.peek_current() != terminator {
            if self.peek_current() == b'\0' { return self.get_invalid(LexError::IncompleteString) };
            if self.peek_current() == b'\\' {
                self.advance_current();
                self.advance_current();
                continue;
            }

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

    fn handle_char_lit(&mut self) -> Token<'a> {
        self.advance_current();

        let char = self.peek_current();
        let char_starting_index = self.current;

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
        let end_index = self.current;
        if self.peek_current() != b'\'' { return self.get_invalid(LexError::InvalidCharLiteral)};
        self.advance_current();

        match Self::get_char_size(char) {
            1 => Token::Char8Lit(char),
            2 => Token::Char16Lit(&self.source[char_starting_index..end_index]),
            _ => Token::Char32Lit(&self.source[char_starting_index..end_index])
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

        self.source[self.current + till]
    }

    fn matches_current(&self, pattern: &str) -> bool {
        let bytes = pattern.as_bytes();
        let end = self.current + bytes.len();
        if end > self.source.len() {
            return false;
        }

        &self.source[self.current..end] == bytes
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

    pub fn get_char_size(byte: u8) -> usize {
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
