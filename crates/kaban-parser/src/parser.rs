use kaban_core::{SourceIndex, SourceSpan, ToSourceIndex, ToUsize, source::Source};
use kaban_lexer::{Token, token::TokenKind};
use crate::{ast::{ExtraIndex, NodeData, NodeIndex, NodeTag, SourceIndexVec, TokenIndex}, errors::ParseError};

pub struct Parser<'a> {
    tokens: &'a [Token],
    source: Source<'a>,
    current: usize,
    pub errors: Vec<ParseError>,

    pub node_tags: Vec<NodeTag>,
    pub node_data: Vec<NodeData>,
    pub extra: Vec<SourceIndex>,
}

pub struct AST<'a> {
    pub tokens: &'a [Token],
    pub node_tags: Vec<NodeTag>,
    pub node_data: Vec<NodeData>,
    pub extra: Vec<SourceIndex>,
    pub source: Source<'a>,
    pub root: NodeIndex,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], source: Source<'a>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
            source,
            
            node_tags: Vec::new(),
            node_data: Vec::new(),
            extra: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> AST<'a> {
        let mut top_level_statements  = Vec::new();
        while !self.is_at_end() {
            if let Some(statment) = self.parse_next_statement() {
                top_level_statements.push(statment);
            }
        };

        let root = self.push_block(top_level_statements);
        AST {
            tokens: self.tokens,
            node_tags: std::mem::take(&mut self.node_tags),
            node_data: std::mem::take(&mut self.node_data),
            extra: std::mem::take(&mut self.extra),
            source: self.source,
            root,
        }
    }
    
    pub fn reset(&mut self, tokens: &'a [Token], source: Source<'a>) {
        //these should be clear via mem take but its better to be safe
        self.node_tags.clear();
        self.node_data.clear();
        self.extra.clear();

        self.tokens = tokens;
        self.source = source;
        self.current = 0;
    }

    pub fn parse_next_statement(&mut self) -> Option<NodeIndex> {
        let current_token = self.peek_current();

        match current_token.kind {
            TokenKind::Let => self.handle_let_statements(),
            _ => { 
                let expression = self.parse_expression(); 
                self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon);
                expression
            },
        }
    }

    pub fn parse_expression(&mut self) -> Option<NodeIndex> {
        self.continue_parsing_expression(0)
    }

    fn push_node(&mut self, tag: NodeTag, left: SourceIndex, right: SourceIndex) -> NodeIndex {
        let index = self.node_tags.len();
        self.node_tags.push(tag);
        self.node_data.push(NodeData { left, right });
        NodeIndex(index as SourceIndex)
    }

    fn push_one_extra(&mut self, data: SourceIndex) -> ExtraIndex {
        let starting_index = self.extra.len().source_index();
        self.extra.push(data);
        ExtraIndex(starting_index)
    }

    fn push_extra(&mut self, data: &[SourceIndex]) -> ExtraIndex {
        let starting_index = self.extra.len().source_index();
        for element in data.iter().copied() {
            self.extra.push(element);
        };
        ExtraIndex(starting_index)
    }

    fn continue_parsing_expression(&mut self, left_precedence_level: u8) -> Option<NodeIndex> {
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

            if matches!(new_operator, NodeTag::As) {
                let type_ = self.parse_type_decleration()?;
                left_side = self.push_node(NodeTag::As, left_side.0, type_.0);
                continue;
            }

            let right_side = self.continue_parsing_expression(new_operator.precedence())?;
            left_side = match new_operator {
                op if op.is_member_access() => self.parse_member_access_or_method(left_side, right_side, op)?,
                NodeTag::As => unreachable!(),
                op if op.is_prefix() => unreachable!(),
                op if op.is_postfix() => unreachable!(),
                NodeTag::Index => unreachable!(),
                NodeTag::FuncCall => unreachable!(),
                _ => self.push_node(new_operator, left_side.0, right_side.0)
            };
        };

        Some(left_side)
    }

    fn consume_atom_or_prefix_unary(&mut self) -> Option<NodeIndex> {
        if let Some(prefix_unary) = self.try_consume_prefix_unary_operator() {
            return self.parse_prefix_unary_expression(prefix_unary);
        };

        let current_token = self.peek_current();
        let left = current_token.index;
        let right = 0;
        let mut advance_after_match = true;
        let atom = match current_token.kind {
            TokenKind::IntLit => self.push_node(NodeTag::IntLit, left.0, right),
            TokenKind::FloatLit => self.push_node(NodeTag::FloatLit, left.0, right),
            TokenKind::Identifier => self.push_node(NodeTag::Identifier, left.0, right),
            TokenKind::BoolLit => {
                let bool: SourceIndex = if self.source.matches(current_token.span(), "true") { 1 } else { 0 };
                self.push_node(NodeTag::BoolLit, bool, right)
            },
            TokenKind::StringLit => self.push_node(NodeTag::StringLit, left.0, right),
            TokenKind::LeftBracket => {
                advance_after_match = false;
                self.advance();
                let args = self.parse_comma_seperated_expressions(TokenKind::RightBracket);
                self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket);
                let right = self.push_extra(args.source_index());
                self.push_node(NodeTag::ArrayLit, args.len().source_index(), right.0)
            }
            TokenKind::StringObjLit => todo!(),
            TokenKind::InterpolatedStringObjLit => todo!(),
            TokenKind::LeftParen => {
                advance_after_match = false;
                self.advance();
                let parenthesis_expression = self.parse_expression()?;
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                parenthesis_expression
            },
            TokenKind::Undefined => self.push_node(NodeTag::Undefined, left.0, right),
            TokenKind::Garbage => self.push_node(NodeTag::Garbage, left.0, right),
            TokenKind::Self_ => self.push_node(NodeTag::Self_, left.0, right),
            _ => {
                self.error_recovery(ParseError::Expected("Expression".to_string()));
                return None;
            },
        };

        if advance_after_match {
            self.advance();
        };
        Some(atom)
    }

    fn parse_prefix_unary_expression(&mut self, prefix_unary: NodeTag) -> Option<NodeIndex> {
        let operand = self.parse_expression()?;
        match prefix_unary {
            NodeTag::New => todo!(),
            NodeTag::Destruct => todo!(),
            _ => self.push_node(prefix_unary, operand.0, 0).some(),
        }
    }

    fn parse_postfix_expression(&mut self, operand: NodeIndex, operator: NodeTag)-> Option<NodeIndex> {
        match  operator {
            NodeTag::Deref |
            NodeTag::Bang |
            NodeTag::Question => self.push_node(operator, operand.0, 0).some(),
            NodeTag::FuncCall => {
                let args = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                self.must_consume(TokenKind::RightParen, ParseError::Expected(")".to_string()))?;
                let extra_index = self.push_one_extra(args.len().source_index());
                self.push_extra(args.source_index());
                self.push_node(NodeTag::FuncCall, operand.0, extra_index.0).some()
            },
            NodeTag::Index => {
                let index = self.parse_expression()?;
                let safe = !self.if_matches_then_consume_bool(TokenKind::Bang);
                self.must_consume(TokenKind::RightBracket, ParseError::Expected("]".to_string()))?;
                let extra = self.push_one_extra(safe.source_index());
                self.push_one_extra(index.0);
                self.push_node(NodeTag::Index, operand.0, extra.0).some()
            }
            _ => unreachable!(),
        }
    }
    fn handle_let_statements(&mut self) -> Option<NodeIndex> {
        // self.advance();
        // let mutable = self.if_matches_then_consume_bool(TokenKind::Mut);
        // let name = self.must_consume(TokenKind::Identifier, ParseError::Expected("Identifier".to_string()))?;
        // let name = name.span;
        // let let_type = if self.if_matches_then_consume_bool(TokenKind::Colon) {
        //     self.parse_type_decleration()
        // } else { 
        //     None 
        // };
        // self.must_consume(TokenKind::Equals, ParseError::Expected("=".to_string()))?;
        // let assignment = self.parse_expression()?;
        // self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
        // Some(Statement::Let { 
        //     mutable, 
        //     name, 
        //     let_type, 
        //     assignment,
        // })
        todo!()
    }

    fn parse_type_decleration(&mut self) -> Option<NodeIndex> {
        let current_token = self.peek_current();
        let mut advance_after_match = true;
        let mut type_ = match current_token.kind {
            TokenKind::I8 => self.push_node(NodeTag::I8, 0, 0),
            TokenKind::I16 => self.push_node(NodeTag::I16, 0, 0),
            TokenKind::I32 => self.push_node(NodeTag::I32, 0, 0),
            TokenKind::I64 => self.push_node(NodeTag::I64, 0, 0),
            TokenKind::F32 => self.push_node(NodeTag::F32, 0, 0),
            TokenKind::F64 => self.push_node(NodeTag::F64, 0, 0),
            TokenKind::U8 => self.push_node(NodeTag::U8, 0, 0),
            TokenKind::U16 => self.push_node(NodeTag::U16, 0, 0), 
            TokenKind::U32 => self.push_node(NodeTag::U32, 0, 0), 
            TokenKind::U64 => self.push_node(NodeTag::U64, 0, 0),
            TokenKind::USize => self.push_node(NodeTag::USize, 0, 0),
            TokenKind::C8 => self.push_node(NodeTag::C8, 0, 0),
            TokenKind::C16 => self.push_node(NodeTag::C16, 0, 0),
            TokenKind::C32 => self.push_node(NodeTag::C32, 0, 0),
            TokenKind::Bool => self.push_node(NodeTag::Bool, 0, 0),
            TokenKind::Void => self.push_node(NodeTag::Void, 0, 0),
            TokenKind::Undefined => self.push_node(NodeTag::Undefined, 0, 0),
            TokenKind::Garbage => self.push_node(NodeTag::Garbage, 0, 0),
            TokenKind::Identifier => self.push_node(NodeTag::Named, current_token.index.0, 0),
            TokenKind::Union => {
                advance_after_match = false;
                self.advance();
                self.must_consume(TokenKind::LeftParen, ParseError::MissingLeftParen)?;
                let mut types = Vec::new();
                while !self.is_at_end() && !self.if_matches_then_consume_bool(TokenKind::RightParen) {
                    types.push(self.parse_type_decleration()?);
                    if !self.if_matches_then_consume_bool(TokenKind::Comma) { break; }
                }
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                let extra = self.push_extra(types.source_index());
                self.push_node(NodeTag::Union, types.len().source_index(), extra.0)
            }
            _ => {
                self.error_recovery(ParseError::MissingTypeDeclaration);
                return None;
            },
        };

        if advance_after_match {
            self.advance();
        }

        loop {
            advance_after_match = true;
            type_ = match  self.peek_current().kind {
                TokenKind::Star => self.push_node(NodeTag::Pointer, type_.0, 0),
                TokenKind::Ampersand => self.push_node(NodeTag::Borrow, type_.0, 0),
                TokenKind::AmpersandMut => self.push_node(NodeTag::MutBorrow, type_.0, 0),
                TokenKind::Question => self.push_node(NodeTag::Optional, type_.0, 0),
                TokenKind::Bang => self.push_node(NodeTag::OptionalGarbage, type_.0, 0),
                TokenKind::LeftBracket => {
                    advance_after_match = false;
                    self.advance();
                    if matches!(self.peek_current().kind, TokenKind::RightBracket) {
                        self.advance();
                        self.push_node(NodeTag::DynArray, type_.0, 0)
                    } else {
                        let size = self.parse_expression()?;
                        self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket)?;
                        self.push_node(NodeTag::FixedArray, type_.0, size.0)
                    }
                }
                _ => break,
            };

            if advance_after_match {
                self.advance();
            };
        }

        Some(type_)
    }

    fn parse_member_access_or_method(&mut self, parent: NodeIndex, child: NodeIndex, operator: NodeTag) -> Option<NodeIndex> {
        match operator {
            NodeTag::Dot | NodeTag::Colon => {
                if self.if_matches_then_consume_bool(TokenKind::LeftParen) {
                    let args  = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    let mutable_self = operator == NodeTag::Colon;
                    let extra = self.push_one_extra(mutable_self.source_index());
                    self.push_one_extra(args.len().source_index());
                    self.push_extra(args.source_index());
                    self.push_node(NodeTag::MethodCall, parent.0, extra.0).some()
                } else {
                    self.push_node(operator, parent.0, child.0).some()
                }
            }
            _ => self.push_node(operator, parent.0, child.0).some()
        }
    }

    fn push_block(&mut self, statements: Vec<NodeIndex>) -> NodeIndex {
        let block_size = statements.len().source_index();
        let extra_ptr = self.push_extra(statements.source_index());
        self.push_node(NodeTag::Block, block_size, extra_ptr.0)
    }

}

//helper
impl<'a> Parser<'a> {
    fn peek_current(&self) -> TokenRef<'a> {
        self.peek_offset(0)
    }

    // fn peek_next(&self) -> &'a Token {
    //     self.peek_offset(1)
    // }

    fn peek_offset(&self, offset: usize) -> TokenRef<'a> {
        let index = self.current + offset;
        let token = &self.tokens[index];
        TokenRef { token, index: TokenIndex(index as SourceIndex), kind: token.kind }
    }

    fn advance(&mut self) -> TokenRef<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }

        // &self.tokens[self.current - 1]
        let index = self.current - 1;
        let token = &self.tokens[index];
        TokenRef { token, index: TokenIndex(index as SourceIndex), kind: token.kind }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.tokens[self.current].kind, TokenKind::EOF)
    }

    fn check(&mut self, token_kind: TokenKind) -> Option<TokenRef<'a>> {
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
    fn must_check(&mut self, token: TokenKind, error: ParseError) -> Option<TokenRef<'a>> {
        match self.check(token) {
            Some(found_token) => Some(found_token),
            None => {
                self.error_recovery(error);
                None
            }
        }
    }

    fn must_consume(&mut self, token_kind: TokenKind, error: ParseError) -> Option<TokenRef<'a>> {
        match self.must_check(token_kind, error) {
            Some(_) => Some(self.advance()),
            None => None
        }
    }

    /**
     * If token matches expected, this advances, stays in place otherwise
     */
    fn if_matches_then_consume(&mut self, token_kind: TokenKind) -> Option<TokenRef<'a>> {
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

    fn peek_infix_or_postfix_operator(&mut self) -> Option<NodeTag> {
        let current_token = self.peek_current();
        Some(match current_token.kind {
            TokenKind::Plus => NodeTag::Add,
            TokenKind::Minus => NodeTag::Subtract,
            TokenKind::Star => NodeTag::Multiply,
            TokenKind::Slash => NodeTag::Divide,
            TokenKind::Percent => NodeTag::Modulo,
            TokenKind::EqualEqual => NodeTag::Equal,
            TokenKind::BangEqual => NodeTag::NotEqual,
            TokenKind::Less => NodeTag::Less,
            TokenKind::Greater => NodeTag::Greater,
            TokenKind::LessEqual => NodeTag::LessEqual,
            TokenKind::GreaterEqual => NodeTag::GreaterEqual,
            TokenKind::And => NodeTag::And,
            TokenKind::Or => NodeTag::Or,
            TokenKind::Band => NodeTag::BAnd,
            TokenKind::Bor => NodeTag::BOr,
            TokenKind::Bxor => NodeTag::XOr,
            TokenKind::LessLess => NodeTag::LeftShift,
            TokenKind::GreaterGreater => NodeTag::RightShift,
            TokenKind:: GreaterGreaterGreater => NodeTag::UnsignedRightShift,
            TokenKind::Caret => NodeTag::Deref,
            TokenKind::Bang => NodeTag::Bang,
            TokenKind::Question => NodeTag::Question,
            TokenKind::Dot => NodeTag::Dot,
            TokenKind::BangDot => NodeTag::ExclamationDot,
            TokenKind::QuestionDot => NodeTag::QuestionDot,
            TokenKind::Colon => NodeTag::Colon,
            TokenKind::QuestionQuestionDot => NodeTag::QuestionQuestionDot,
            TokenKind::LeftBracket => NodeTag::UndefinedCoalescing,
            TokenKind::As => NodeTag::As,
            TokenKind::LeftParen => NodeTag::FuncCall,
            _ => return None,
        })
    }

    fn try_consume_infix_or_postfix_operator(&mut self) -> Option<NodeTag> {
        let operator = self.peek_infix_or_postfix_operator();
        self.advance();
        operator
    }

    pub fn try_consume_prefix_unary_operator(&mut self) -> Option<NodeTag> {
        let current_token = self.peek_current();
        let operator = match current_token.kind {
            TokenKind::Minus => NodeTag::Negative,
            TokenKind::Bang => NodeTag::Not,
            TokenKind::Bnot => NodeTag::BNot,
            TokenKind::New => NodeTag::New,
            TokenKind::Destruct => NodeTag::Destruct,
            _ => return None,
        };
        self.advance();
        Some(operator)
    }

    fn parse_comma_seperated_expressions(&mut self, closing_delimiter: TokenKind) -> Vec<NodeIndex> {
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


struct TokenRef<'a> {
    token: &'a Token,
    index: TokenIndex,
    kind: TokenKind,
}

impl<'a> TokenRef<'a> {
    // fn unwrap(&self) -> &'a Token {
    //     self.token
    // }

    // #[inline(always)]
    // fn kind(&self) -> TokenKind {
    //     self.token.kind
    // }

    #[inline(always)]
    pub fn span(&self) -> SourceSpan {
        self.token.span
    }
}
