use kaban_lexer::{Lexer, Token, token::TokenKind};
use crate::{ast::{Expression, Statement, Type}, errors::ParseError, operator::{self, Arithmetic, BitwiseBinary, Comparison, HasPrecedence, Logical, MemberAccess, Operator, PostfixUnary, PrefixUnary, Special}};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    pub errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse_tokens(&mut self) -> Expression {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.parse_next_statement() {
                Some(statement) => statements.push(statement),
                None => continue,
            }
        };

        Expression::Block { statements: statements, value: None }
    }

    pub fn parse_next_statement(&mut self) -> Option<Statement> {
        let current_token = self.peek_current();

        match current_token.kind {
            TokenKind::Let => self.handle_let_statements(),
            _ => Some(Statement::ExpressionStatement(self.parse_expression()?))
        }
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        self.continue_parsing_expression(0)
    }

    fn continue_parsing_expression(&mut self, left_precedence_level: u8) -> Option<Expression> {
        let mut left_side  = self.consume_atom_or_prefix_unary()?;

        while let Some(new_operator) = self.peek_infix_or_postfix_operator() {
            if left_precedence_level >= new_operator.precedence() {
                break;
            };

            let new_operator = self.try_consume_infix_or_postfix_operator()?;

            if new_operator.is_postfix() {
                left_side = self.parse_postfix_expression(left_side, new_operator)?;
                continue;
            };

            let left = left_side.to_box();
            if matches!(new_operator, Operator::Special(Special::As)) {
                left_side = Expression::TypeCasting { value: left, type_: self.parse_type_decleration()? };
                continue;
            }

            let right = self.continue_parsing_expression(new_operator.precedence())?.to_box();
            left_side = match new_operator {
                Operator::Arithmetic(operator) => Expression::ArithmeticOperation {left, right, operator},
                Operator::Comparison(operator) => Expression::ComparisonOperation { left, right, operator },
                Operator::Logical(operator) => Expression::LogicalOperation {left, right, operator},
                Operator::BitwiseBinary(operator) => Expression::BinaryOperation { left, right, operator },
                Operator::MemberAccess(operator) => self.parse_member_access_or_method(left, right, operator)?,
                Operator::Special(operator) => match operator {
                    Special::UndefinedCoalescing => Expression::UndefinedCoalescing { possibly_undefined: left, default: right },
                    Special::As => unreachable!(),
                },
                Operator::PrefixUnary(_) => unreachable!(),
                Operator::PostfixUnary(_) => unreachable!(),
                Operator::Index => unreachable!(),
                Operator::FuncCall => unreachable!(),
            };
        };

        Some(left_side)
    }

    fn consume_atom_or_prefix_unary(&mut self) -> Option<Expression> {
        if let Some(prefix_unary) = self.try_consume_prefix_unary_operator() {
            return self.parse_prefix_unary_expression(prefix_unary);
        };

        let current_token = self.peek_current();
        let atom = match current_token.kind {
            TokenKind::IntLit => { self.advance(); Expression::IntLit(current_token.span) },
            TokenKind::FloatLit => { self.advance(); Expression::FloatLit(current_token.span) },
            TokenKind::Identifier => { self.advance(); Expression::Identifier(current_token.span) },
            TokenKind::BoolLit => { self.advance(); Expression::BoolLit(current_token.span) },
            TokenKind::StringLit => { self.advance(); Expression::StringLit(current_token.span) },
            TokenKind::LeftBracket => {
                self.advance();
                let args = self.parse_comma_seperated_expressions(TokenKind::RightBracket);
                self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket);
                Expression::ArrayLit(args)
            }
            TokenKind::StringObjLit => todo!(),
            TokenKind::InterpolatedStringObjLit => todo!(),
            TokenKind::LeftParen => {
                self.advance();
                let expression = self.parse_expression()?;
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                expression
            },
            TokenKind::Undefined => { self.advance(); Expression::Undefined },
            TokenKind::Garbage => { self.advance(); Expression::Garbage },
            TokenKind::Self_ => { self.advance(); Expression::Self_ },
            _ => {
                self.error_recovery(ParseError::Expected("Expression".to_string()));
                return None;
            },
        };

        Some(atom)
    }

    fn parse_prefix_unary_expression(&mut self, prefix_unary: PrefixUnary) -> Option<Expression> {
        match prefix_unary {
            PrefixUnary::New => todo!(),
            PrefixUnary::Destruct => todo!(),
            _ => Expression::PrefixUnaryOperation { 
                operand: self.continue_parsing_expression(prefix_unary.precedence())?.to_box(),
                operator: prefix_unary,
            }.to_some(),
        }
    }

    fn parse_postfix_expression(&mut self, operand: Expression, operator: Operator)-> Option<Expression> {
        match  operator {
            Operator::PostfixUnary(operator) => Expression::PostfixUnaryOperation { operand: operand.to_box(), operator }.to_some(),
            Operator::FuncCall => {
                let args = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                self.must_consume(TokenKind::RightParen, ParseError::Expected(")".to_string()))?;
                Expression::FunctionCall { callee: operand.to_box(), args }.to_some()
            },
            Operator::Index => {
                let index = self.parse_expression()?.to_box();
                let safe = !self.if_matches_then_consume_bool(TokenKind::Bang);
                self.must_consume(TokenKind::RightBracket, ParseError::Expected("]".to_string()))?;
                Expression::IndexOperation { parent: operand.to_box(), index, safe }.to_some()
            }
            _ => unreachable!(),
        }
    }
    fn handle_let_statements(&mut self) -> Option<Statement> {
        self.advance();
        let mutable = self.if_matches_then_consume_bool(TokenKind::Mut);
        let name = self.must_consume(TokenKind::Identifier, ParseError::Expected("Identifier".to_string()))?;
        let name = name.span;
        let let_type = if self.if_matches_then_consume_bool(TokenKind::Colon) {
            self.parse_type_decleration()
        } else { 
            None 
        };
        self.must_consume(TokenKind::Equals, ParseError::Expected("=".to_string()))?;
        let assignment = self.parse_expression()?;
        self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
        Some(Statement::Let { 
            mutable, 
            name, 
            let_type, 
            assignment,
        })
    }

    fn parse_type_decleration(&mut self) -> Option<Type> {
        let current_token = self.peek_current();
        let mut type_ = match current_token.kind {
            TokenKind::I8 => { self.advance(); Type::I8 },
            TokenKind::I16 => { self.advance(); Type::I16 },
            TokenKind::I32 => { self.advance(); Type::I32 },
            TokenKind::I64 => { self.advance(); Type::I64 },
            TokenKind::F32 => { self.advance(); Type::F32 },
            TokenKind::F64 => { self.advance(); Type::F64 },
            TokenKind::U8 => { self.advance(); Type::U8 },
            TokenKind::U16 => { self.advance(); Type::U16 }, 
            TokenKind::U32 => { self.advance(); Type::U32 }, 
            TokenKind::U64 => { self.advance(); Type::U64 },
            TokenKind::USize => { self.advance(); Type::USize },
            TokenKind::C8 => { self.advance(); Type::C8 },
            TokenKind::C16 => { self.advance(); Type::C16 },
            TokenKind::C32 => { self.advance(); Type::C32 },
            TokenKind::Bool => { self.advance(); Type::Bool },
            TokenKind::Void => { self.advance(); Type::Void },
            TokenKind::Undefined => { self.advance(); Type::Undefined },
            TokenKind::Garbage => { self.advance(); Type::Garbage },
            TokenKind::Identifier => { self.advance(); Type::Named(current_token.span) },
            TokenKind::Union => {
                self.advance();
                self.must_consume(TokenKind::LeftParen, ParseError::MissingLeftParen)?;
                let mut types = Vec::new();
                while !self.is_at_end() && !self.if_matches_then_consume_bool(TokenKind::RightParen) {
                    types.push(self.parse_type_decleration()?);
                    if !self.if_matches_then_consume_bool(TokenKind::Comma) { break; }
                }
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                Type::Union(types)
            }
            _ => {
                self.error_recovery(ParseError::MissingTypeDeclaration);
                return None;
            },
        };

        loop {
            type_ = match  self.peek_current().kind {
                TokenKind::Star => { self.advance(); Type::Pointer(type_.to_box()) },
                TokenKind::Ampersand => { self.advance(); Type::Borrow(type_.to_box()) },
                TokenKind::AmpersandMut => { self.advance(); Type::MutBorrow(type_.to_box()) },
                TokenKind::Question => { self.advance(); Type::Optional(type_.to_box()) },
                TokenKind::Bang => { self.advance(); Type::OptionalGarbage(type_.to_box()) },
                TokenKind::LeftBracket => {
                    self.advance();
                    if matches!(self.peek_current().kind, TokenKind::RightBracket) {
                        self.advance();
                        Type::DynArray(type_.to_box())
                    } else {
                        let size = self.parse_expression()?.to_box();
                        self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket)?;
                        Type::FixedArray { type_: type_.to_box(), size }
                    }
                }
                _ => break,
            }
        }

        Some(type_)
    }

    fn parse_member_access_or_method(&mut self, parent: Box<Expression>, child: Box<Expression>, operator: MemberAccess) -> Option<Expression> {
        let parent = parent.to_box();
        let method_name = match *child {
            Expression::Identifier(s) => s,
            _ => {
                self.error_recovery(ParseError::InvalidMethodName);
                return None;
            }
        };

        match operator {
            MemberAccess::Dot | MemberAccess::Colon => {
                if self.if_matches_then_consume_bool(TokenKind::LeftParen) {
                    let args  = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    Expression::MethodCall {
                        parent,
                        method_name,
                        args,
                        mutable_self: operator == MemberAccess::Colon,
                    }.to_some()
                } else {
                    Expression::MemberAccess { parent, child, operator }.to_some()
                }
            }
            _ => Expression::MemberAccess { parent, child, operator }.to_some()
        }
    }

}

//helper
impl<'a> Parser<'a> {
    fn peek_current(&self) -> &'a Token {
        self.peek_offset(0)
    }

    // fn peek_next(&self) -> &'a Token {
    //     self.peek_offset(1)
    // }

    fn peek_offset(&self, offset: usize) -> &'a Token {
        &self.tokens[self.current + offset]
    }

    fn advance(&mut self) -> &'a Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.tokens[self.current].kind, TokenKind::EOF)
    }

    fn check(&mut self, token_kind: TokenKind) -> Option<&'a Token> {
        let current = self.peek_current();
        if current.kind != token_kind {
            None
        } else {
            Some(current)
        }
    }

    fn check_bool(&mut self, token_kind: TokenKind) -> bool {
        self.check(token_kind).is_some()
    }

    /**
     * Returns token if found, does error recovery and logs error if not
     */
    fn must_check(&mut self, token: TokenKind, error: ParseError) -> Option<&'a Token> {
        match self.check(token) {
            Some(found_token) => Some(found_token),
            None => {
                self.error_recovery(error);
                None
            }
        }
    }

    fn must_consume(&mut self, token_kind: TokenKind, error: ParseError) -> Option<&'a Token> {
        match self.must_check(token_kind, error) {
            Some(_) => Some(self.advance()),
            None => None
        }
    }

    /**
     * If token matches expected, this advances, stays in place otherwise
     */
    fn if_matches_then_consume(&mut self, token_kind: TokenKind) -> Option<&'a Token> {
        if let Some(found_token) = self.check(token_kind) {
            self.advance();
            Some(found_token)
        } else {
            None
        }
    }

    fn if_matches_then_consume_bool(&mut self, token_kind: TokenKind) -> bool {
        self.if_matches_then_consume(token_kind).is_some()
    }

    // fn consume_identifier(&mut self) -> Option<&'a str>{
    //     match self.peek_current() {
    //         Token::Identifier(name) => Some(name), 
    //         _ => {self.error_recovery(ParseError::Expected("Identifier".to_string())); None}
    //     }
    // }

    fn error_recovery(&mut self, error: ParseError) {
        self.errors.push(error);
        while !Self::is_recovery_point(self.peek_current().kind) && !self.is_at_end() {
            self.advance();
        }
    }

    fn is_recovery_point(token: TokenKind) -> bool {
        token == TokenKind::Semicolon ||
            token == TokenKind::RightBrace ||
            token == TokenKind::Pub ||
            token == TokenKind::Func ||
            token == TokenKind::EOF
    }

    fn peek_infix_or_postfix_operator(&mut self) -> Option<Operator> {
        let current_token = self.peek_current();
        let operator = match current_token.kind {
            TokenKind::Plus => Operator::Arithmetic(Arithmetic::Add),
            TokenKind::Minus => Operator::Arithmetic(Arithmetic::Subtract),
            TokenKind::Star => Operator::Arithmetic(Arithmetic::Multiply),
            TokenKind::Slash => Operator::Arithmetic(Arithmetic::Divide),
            TokenKind::Percent => Operator::Arithmetic(Arithmetic::Modulo),
            TokenKind::EqualEqual => Operator::Comparison(Comparison::Equal),
            TokenKind::BangEqual => Operator::Comparison(Comparison::NotEqual),
            TokenKind::Less => Operator::Comparison(Comparison::Less),
            TokenKind::Greater => Operator::Comparison(Comparison::Greater),
            TokenKind::LessEqual => Operator::Comparison(Comparison::LessEqual),
            TokenKind::GreaterEqual => Operator::Comparison(Comparison::GreaterEqual),
            TokenKind::And => Operator::Logical(Logical::And),
            TokenKind::Or => Operator::Logical(Logical::Or),
            TokenKind::Band => Operator::BitwiseBinary(BitwiseBinary::And),
            TokenKind::Bor => Operator::BitwiseBinary(BitwiseBinary::Or),
            TokenKind::Bxor => Operator::BitwiseBinary(BitwiseBinary::Xor),
            TokenKind::LessLess => Operator::BitwiseBinary(BitwiseBinary::LeftShift),
            TokenKind::GreaterGreater => Operator::BitwiseBinary(BitwiseBinary::RightShift),
            TokenKind:: GreaterGreaterGreater => Operator::BitwiseBinary(BitwiseBinary::UnsignedRightShift),
            TokenKind::Caret => Operator::PostfixUnary(PostfixUnary::Deref),
            TokenKind::Bang => Operator::PostfixUnary(PostfixUnary::Bang),
            TokenKind::Question => Operator::PostfixUnary(PostfixUnary::Question),
            TokenKind::Dot => Operator::MemberAccess(MemberAccess::Dot),
            TokenKind::BangDot => Operator::MemberAccess(MemberAccess::ExclamationDot),
            TokenKind::QuestionDot => Operator::MemberAccess(MemberAccess::QuestionDot),
            TokenKind::Colon => Operator::MemberAccess(MemberAccess::Colon),
            TokenKind::QuestionQuestionDot => Operator::MemberAccess(MemberAccess::QuestionQuestionDot),
            TokenKind::LeftBracket => Operator::Index,
            TokenKind::QuestionQuestion => Operator::Special(Special::UndefinedCoalescing),
            TokenKind::As => Operator::Special(Special::As),
            TokenKind::LeftParen => Operator::FuncCall,
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
        let operator = match current_token.kind {
            TokenKind::Minus => PrefixUnary::Negative,
            TokenKind::Bang => PrefixUnary::Not,
            TokenKind::Bnot => PrefixUnary::BNot,
            TokenKind::New => PrefixUnary::New,
            TokenKind::Destruct => PrefixUnary::Destruct,
            _ => return None,
        };
        self.advance();
        Some(operator)
    }

    fn parse_comma_seperated_expressions(&mut self, closing_delimiter: TokenKind) -> Vec<Expression> {
        let mut expressions = Vec::new();
        while !self.is_at_end() && !self.check_bool(closing_delimiter) {
            if let Some(expression) = self.parse_expression() {
                expressions.push(expression);
            };

            if !self.if_matches_then_consume_bool(TokenKind::Comma) {break;};
        }

        expressions
    }

    // DEAD CODE:
    // fn parse_right_side_expression(
    //     &mut self, 
    //     left_side: Expression, 
    //     left_operator: Operator, 
    // ) -> Option<Expression> {
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

    //Old code for desugaring string lits
                // let mut items = Vec::new();
                // let bytes = s.as_bytes();
                // let mut i: usize = 0;
                // while i < bytes.len() {
                //     let char = bytes[i];
                //     match Lexer::get_char_size(char) {
                //         1 => {
                //             items.push(Expression::Char8Lit(char));
                //             i += 1;
                //         },
                //         2 => {
                //             items.push(Expression::Char16Lit(&bytes[i..i+2]));
                //             i += 2;
                //         },
                //         3 => {
                //             items.push(Expression::Char32Lit(&bytes[i..i+3]));
                //             i += 3;
                //         },
                //         _ => { 
                //             items.push(Expression::Char32Lit(&bytes[i..i+4]));
                //             i += 4;
                //         },
                //     }
                // }
                // self.advance();
                // Expression::ArrayLit(items)
}
