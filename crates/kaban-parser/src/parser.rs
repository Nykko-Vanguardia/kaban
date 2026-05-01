use kaban_lexer::{Lexer, Token};
use crate::{ast::{Expression, Statement, Type}, errors::ParseError, operator::{Arithmetic, BitwiseBinary, Comparison, HasPrecedence, Index, Logical, MemberAccess, Operator, PostfixUnary, PrefixUnary, Special}};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse_tokens(&mut self) -> Expression<'a> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.parse_next_statement() {
                Some(statement) => statements.push(statement),
                None => continue,
            }
        };

        Expression::Block { statements: statements, value: None }
    }

    fn parse_next_statement(&mut self) -> Option<Statement<'a>> {
        let current_token = self.peek_current();

        match current_token {
            Token::Let => self.handle_let_statements(),
            _ => Some(Statement::ExpressionStatement(self.parse_expression()?))
        }
    }

    fn parse_expression(&mut self) -> Option<Expression<'a>> {
        self.continue_parsing_expression(0)
    }

    fn continue_parsing_expression(&mut self, left_precedence_level: u8) -> Option<Expression<'a>> {
        let mut left_side  = self.consume_atom_or_prefix_unary()?;

        while let Some(new_operator) = self.peek_infix_or_postfix_operator() {
            if left_precedence_level >= new_operator.precedence() {
                break;
            };

            let new_operator = self.try_consume_infix_or_postfix_operator()?;

            if new_operator.is_postfix() {
                todo!("func calls and such");
            };

            let left = left_side.to_box();
            let right = self.continue_parsing_expression(new_operator.precedence())?.to_box();
            left_side = match new_operator {
                Operator::Arithmetic(operator) => Expression::ArithmeticOperation {left, right, operator},
                Operator::Comparison(operator) => Expression::ComparisonOperation { left, right, operator },
                Operator::Logical(operator) => Expression::LogicalOperation {left, right, operator},
                Operator::BitwiseBinary(operator) => Expression::BinaryOperation { left, right, operator },
                Operator::MemberAccess(operator) => Expression::MemberAccess {parent: left, child: right, operator},
                Operator::Special(operator) => match operator {
                    Special::UndefinedCoalescing => Expression::UndefinedCoalescing { possibly_undefined: left, default: right },
                    // Special::As => Some(Expression::TypeCasting { value: left, type_: right })
                    Special::As => todo!(),
                },
                Operator::PrefixUnary(_) => unreachable!(),
                Operator::PostfixUnary(_) => unreachable!(),
                Operator::Index(_) => unreachable!(),
                Operator::FuncCall => unreachable!(),
            };
        };

        Some(left_side)
    }

    fn consume_atom_or_prefix_unary(&mut self) -> Option<Expression<'a>> {
        if let Some(prefix_unary) = self.try_consume_prefix_unary_operator() {
            return self.parse_prefix_unary_expression(prefix_unary);
        };

        let current_token = self.peek_current();
        let atom = match current_token {
            Token::IntLit(s) => { self.advance(); Expression::IntLit(s) },
            Token::FloatLit(s) => { self.advance(); Expression::FloatLit(s) },
            Token::Identifier(s) => { self.advance(); Expression::Identifier(s) },
            Token::BoolLit(b) => { self.advance(); Expression::BoolLit(*b) },
            Token::StringLit(s) => {
                let mut items = Vec::new();
                let bytes = s.as_bytes();
                let mut i: usize = 0;
                while i < bytes.len() {
                    let char = bytes[i];
                    match Lexer::get_char_size(char) {
                        1 => {
                            items.push(Expression::Char8Lit(char));
                            i += 1;
                        },
                        2 => {
                            items.push(Expression::Char16Lit(&bytes[i..i+2]));
                            i += 2;
                        },
                        3 => {
                            items.push(Expression::Char32Lit(&bytes[i..i+3]));
                            i += 3;
                        },
                        _ => { 
                            items.push(Expression::Char32Lit(&bytes[i..i+4]));
                            i += 4;
                        },
                    }
                }
                self.advance();
                Expression::ArrayLit(items)
            },
            Token::LeftBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.is_at_end() && self.check_bool(&Token::RightBracket) {
                    let item = self.parse_expression()?;
                    items.push(item);
                    if !self.if_matches_then_consume_bool(&Token::Comma) {break;};
                }

                self.must_consume(&Token::RightBracket, ParseError::MissingRightBracket);
                Expression::ArrayLit(items)
            }
            Token::StringObjLit(_) => todo!(),
            Token::InterpolatedStringObjLit(_) => todo!(),
            Token::LeftParen => {
                self.advance();
                let expression = self.parse_expression()?;
                self.must_consume(&Token::RightParen, ParseError::MissingRightParen);
                expression
            },
            Token::Undefined => { self.advance(); Expression::Undefined },
            Token::Garbage => { self.advance(); Expression::Garbage },
            Token::Self_ => { self.advance(); Expression::Self_ },
            _ => {
                self.error_recovery(ParseError::Expected("Expression".to_string()));
                return None;
            },
        };

        Some(atom)
    }

    fn parse_prefix_unary_expression(&mut self, prefix_unary: PrefixUnary) -> Option<Expression<'a>> {
        match prefix_unary {
            PrefixUnary::New => todo!(),
            PrefixUnary::Destruct => todo!(),
            _ => Expression::PrefixUnaryOperation { 
                operand: self.continue_parsing_expression(prefix_unary.precedence())?.to_box(),
                operator: prefix_unary,
            }.to_some(),
        }
    }
    fn handle_let_statements(&mut self) -> Option<Statement<'a>> {
        self.advance();
        let mutable = self.if_matches_then_consume_bool(&Token::Mut);
        let name = self.consume_identifier()?;
        let let_type = if self.if_matches_then_consume_bool(&Token::Colon) {
            self.handle_type_decl()
        } else { 
            None 
        };
        self.must_consume(&Token::Equals, ParseError::Expected("=".to_string()))?;
        let assignment = self.parse_expression()?;
        self.must_consume(&Token::Semicolon, ParseError::MissingSemicolon);
        Some(Statement::Let { 
            mutable, 
            name, 
            let_type, 
            assignment,
        })
    }

    //TODO: Handle other types
    fn handle_type_decl(&mut self) -> Option<Type<'a>> {
        let type_ = match self.peek_current() {
            Token::I8 => Type::I8,
            Token::I16 => Type::I16,
            Token::I32 => Type::I32,
            Token::I64 => Type::I64,
            Token::F32 => Type::F32,
            Token::F64 => Type::F64,
            Token::U8 => Type::U8, 
            Token::U16 => Type::U16, 
            Token::U32 => Type::U32, 
            Token::U64 => Type::U64,
            Token::USize => Type::USize,
            Token::Char8 => Type::Char8,
            Token::Char16 => Type::Char16,
            Token::Char32 => Type::Char32,
            Token::Bool => Type::Bool,
            Token::Void => Type::Void,
            Token::Undefined => Type::Undefined,
            Token::Garbage => Type::Garbage,
            Token::Identifier(name) => Type::Named(name),
            _ => {
                self.error_recovery(ParseError::MissingTypeDeclaration);
                return None;
            },
        };

        self.advance();
        //modifiers
        let type_ = match self.peek_current() {
            Token::Star => Type::Pointer(Box::new(type_)),
            Token::Ampersand => Type::Borrow(Box::new(type_)),
            Token::AmpersandMut => Type::MutBorrow(Box::new(type_)),
            Token::Question => Type::Optional(Box::new(type_)),
            Token::Bang => Type::OptionalGarbage(Box::new(type_)),
            _ => return Some(type_),
        };
        self.advance();
        Some(type_)
    }

}

//helper
impl<'a> Parser<'a> {
    fn peek_current(&self) -> &'a Token<'a> {
        self.peek_offset(0)
    }

    // fn peek_next(&self) -> &'a Token<'a> {
    //     self.peek_offset(1)
    // }

    fn peek_offset(&self, offset: usize) -> &'a Token<'a> {
        &self.tokens[self.current + offset]
    }

    fn advance(&mut self) -> &'a Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }

        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.tokens[self.current], Token::EOF)
    }

    fn check(&mut self, token: &Token<'a>) -> Option<&'a Token<'a>> {
        let current = self.peek_current();
        if current != token {
            None
        } else {
            Some(current)
        }
    }

    fn check_bool(&mut self, token: &Token<'a>) -> bool {
        self.check(token).is_some()
    }

    /**
     * Returns token if found, does error recovery and logs error if not
     */
    fn must_check(&mut self, token: &Token<'a>, error: ParseError) -> Option<&'a Token<'a>> {
        match self.check(token) {
            Some(found_token) => Some(found_token),
            None => {
                self.error_recovery(error);
                None
            }
        }
    }

    fn must_consume(&mut self, token: &Token<'a>, error: ParseError) -> Option<&'a Token<'a>> {
        match self.must_check(token, error) {
            Some(_) => Some(self.advance()),
            None => None
        }
    }

    /**
     * If token matches expected, this advances, stays in place otherwise
     */
    fn if_matches_then_consume(&mut self, token: &Token<'a>) -> Option<&'a Token<'a>> {
        if let Some(found_token) = self.check(token) {
            self.advance();
            Some(found_token)
        } else {
            None
        }
    }

    fn if_matches_then_consume_bool(&mut self, token: &Token<'a>) -> bool {
        self.if_matches_then_consume(token).is_some()
    }

    fn consume_identifier(&mut self) -> Option<&'a str>{
        match self.peek_current() {
            Token::Identifier(name) => Some(name), 
            _ => {self.error_recovery(ParseError::Expected("Identifier".to_string())); None}
        }
    }

    fn error_recovery(&mut self, error: ParseError) {
        self.errors.push(error);
        while !Self::is_recovery_point(self.peek_current()) && !self.is_at_end() {
            self.advance();
        }
    }

    fn is_recovery_point(token: &Token<'a>) -> bool {
        token == &Token::Semicolon ||
            token == &Token::RightBrace ||
            token == &Token::Pub ||
            token == &Token::Func ||
            token == &Token::EOF
    }

    fn peek_infix_or_postfix_operator(&mut self) -> Option<Operator> {
        let current_token = self.peek_current();
        let operator = match current_token {
            Token::Plus => Operator::Arithmetic(Arithmetic::Add),
            Token::Minus => Operator::Arithmetic(Arithmetic::Subtract),
            Token::Star => Operator::Arithmetic(Arithmetic::Multiply),
            Token::Slash => Operator::Arithmetic(Arithmetic::Divide),
            Token::Percent => Operator::Arithmetic(Arithmetic::Modulo),
            Token::EqualEqual => Operator::Comparison(Comparison::Equal),
            Token::BangEqual => Operator::Comparison(Comparison::NotEqual),
            Token::Less => Operator::Comparison(Comparison::Less),
            Token::Greater => Operator::Comparison(Comparison::Greater),
            Token::LessEqual => Operator::Comparison(Comparison::LessEqual),
            Token::GreaterEqual => Operator::Comparison(Comparison::GreaterEqual),
            Token::And => Operator::Logical(Logical::And),
            Token::Or => Operator::Logical(Logical::Or),
            Token::Band => Operator::BitwiseBinary(BitwiseBinary::And),
            Token::Bor => Operator::BitwiseBinary(BitwiseBinary::Or),
            Token::Bxor => Operator::BitwiseBinary(BitwiseBinary::Xor),
            Token::LessLess => Operator::BitwiseBinary(BitwiseBinary::LeftShift),
            Token::GreaterGreater => Operator::BitwiseBinary(BitwiseBinary::RightShift),
            Token:: GreaterGreaterGreater => Operator::BitwiseBinary(BitwiseBinary::UnsignedRightShift),
            Token::Caret => Operator::PostfixUnary(PostfixUnary::Deref),
            Token::Bang => Operator::PostfixUnary(PostfixUnary::Bang),
            Token::Question => Operator::PostfixUnary(PostfixUnary::Question),
            Token::Dot => Operator::MemberAccess(MemberAccess::Dot),
            Token::BangDot => Operator::MemberAccess(MemberAccess::ExclamationDot),
            Token::QuestionDot => Operator::MemberAccess(MemberAccess::QuestionDot),
            Token::QuestionQuestionDot => Operator::MemberAccess(MemberAccess::QuestionQuestionDot),
            Token::LeftBracket => Operator::Index(Index::SafeIndex),
            Token::LeftBracketBang => Operator::Index(Index::UncheckIndex),
            Token::QuestionQuestion => Operator::Special(Special::UndefinedCoalescing),
            Token::As => Operator::Special(Special::As),
            Token::LeftParen => Operator::FuncCall,
            _ => return None,
        };
        Some(operator)
    }

    fn try_consume_infix_or_postfix_operator(&mut self) -> Option<Operator> {
        let operator = self.peek_infix_or_postfix_operator();
        self.advance();
        operator
    }

    pub fn try_consume_prefix_unary_operator(&mut self) -> Option<PrefixUnary> {
        let current_token = self.peek_current();
        let operator = match current_token {
            Token::Minus => PrefixUnary::Negative,
            Token::Bang => PrefixUnary::Not,
            Token::Bnot => PrefixUnary::BNot,
            Token::New => PrefixUnary::New,
            Token::Destruct => PrefixUnary::Destruct,
            _ => return None,
        };
        self.advance();
        Some(operator)
    }

    // DEAD CODE:
    // fn parse_right_side_expression(
    //     &mut self, 
    //     left_side: Expression<'a>, 
    //     left_operator: Operator, 
    // ) -> Option<Expression<'a>> {
    //     let right_side = self.consume_atom_or_prefix_unary()?;
    //     let right_side = if let Some(right_operator) = self.peek_infix_or_postfix_operator() 
    //         && right_operator.precedence() > left_operator.precedence() {
    //             let right_operator = self.try_consume_infix_or_postfix_operator()?;
    //             self.parse_right_side_expression(right_side, right_operator)?
    //     } else {
    //         right_side
    //     };
    //
    //     let right = right_side.to_box();
    //     let left = left_side.to_box();
    //     match left_operator {
    //         Operator::Arithmetic(operator) => Some(Expression::ArithmeticOperation {left, right, operator}),
    //         Operator::Comparison(operator) => Some(Expression::ComparisonOperation { left, right, operator }),
    //         Operator::Logical(operator) => Some(Expression::LogicalOperation {left, right, operator}),
    //         Operator::BitwiseBinary(operator) => Some(Expression::BinaryOperation { left, right, operator }),
    //         Operator::PrefixUnary(operator) => todo!(),
    //         Operator::PostfixUnary(operator) => todo!(),
    //         Operator::MemberAccess(operator) => Some(Expression::MemberAccess {parent: left, child: right, operator}),
    //         Operator::Special(operator) => match operator {
    //             Special::UndefinedCoalescing => Some(Expression::UndefinedCoalescing { possibly_undefined: left, default: right }),
    //             // Special::As => Some(Expression::TypeCasting { value: left, type_: right })
    //             Special::As => todo!(),
    //         }
    //         Operator::FuncCall => todo!(),
    //         Operator::Index(operator) => todo!()
    //     }
    // }
}
