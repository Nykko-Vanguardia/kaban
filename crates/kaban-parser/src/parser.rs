use kaban_core::{SourceSpan, ToUIndex, ToUsize, UIndex, source::Source};
use kaban_lexer::{Token, token::{TokenKind}};
use crate::{ast::AST, errors::ParseError, node::{ExtraIndex, NodeData, NodeIndex, NodeTag, OptionalNode, TokenIndex, U_NONE, UIndexVec}};

pub struct Parser<'a> {
    tokens: &'a [Token],
    source: Source<'a>,
    current: usize,
    pub errors: Vec<ParseError>,

    node_tags: Vec<NodeTag>,
    node_data: Vec<NodeData>,
    extra: Vec<UIndex>,
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
            if let Some(statment) = self.parse_statement() {
                top_level_statements.push(statment);
            }
        };

        let root = self.push_block(top_level_statements);
        AST::new(
            self.tokens,
            std::mem::take(&mut self.node_tags),
            std::mem::take(&mut self.node_data),
            std::mem::take(&mut self.extra),
            self.source,
            root,
        )
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

    pub fn parse_statement(&mut self) -> Option<NodeIndex> {
        let current_token = self.peek_current();

        match current_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            _ => { 
                let expression = self.parse_expression()?; 
                //Decided to remove expression statement wrapper for now
                //rely on pass keywords
                // let expression_statement = self.push_node(NodeTag::ExpressionStatement, expression.0, 0);

                let tag = self.node_tags[expression.0.usize()];
                if !tag.doesnt_require_semicolon() {
                    self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
                }
                // expression_statement.some()
                expression.some()
            },
        }
    }

    pub fn parse_expression(&mut self) -> Option<NodeIndex> {
        self.continue_parsing_expression(0)
    }

    fn push_node(&mut self, tag: NodeTag, left: UIndex, right: UIndex) -> NodeIndex {
        let index = self.node_tags.len();
        self.node_tags.push(tag);
        self.node_data.push(NodeData { left, right });
        NodeIndex(index as UIndex)
    }

    fn push_one_extra(&mut self, data: UIndex) -> ExtraIndex {
        let starting_index = self.extra.len().uindex();
        self.extra.push(data);
        ExtraIndex(starting_index)
    }

    fn push_extra(&mut self, data: &[UIndex]) -> ExtraIndex {
        let starting_index = self.extra.len().uindex();
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
        let token_kind = current_token.kind;
        let atom = match token_kind {
            TokenKind::IntLit => self.push_node(NodeTag::IntLit, left.0, right),
            TokenKind::FloatLit => self.push_node(NodeTag::FloatLit, left.0, right),
            TokenKind::Identifier => self.push_node(NodeTag::Identifier, left.0, right),
            TokenKind::BoolLit => {
                let bool: UIndex = if self.source.matches(current_token.span(), "true") { 1 } else { 0 };
                self.push_node(NodeTag::BoolLit, bool, right)
            },
            TokenKind::LeftBracket => {
                advance_after_match = false;
                self.advance();
                let args = self.parse_comma_seperated_expressions(TokenKind::RightBracket);
                self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket);
                let right = self.push_extra(args.uindex_slice());
                self.push_node(NodeTag::ArrayLit, args.len().uindex(), right.0)
            }
            TokenKind::StringLit => self.push_node(NodeTag::StringLit, left.0, right),
            TokenKind::StringObjLit => todo!(),
            TokenKind::InterpolatedStringObjLit => todo!(),
            TokenKind::LeftParen => {
                advance_after_match = false;
                self.advance();
                let parenthesis_expression = self.parse_expression()?;
                if self.check_bool(TokenKind::Comma) {
                    self.advance();
                    let first_element = parenthesis_expression;
                    let additional_len_for_first_element = 1;
                    let tuple_elements = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    let extra_pointer = self.push_one_extra(first_element.0);
                    self.push_extra(tuple_elements.uindex_slice());
                    self.push_node(NodeTag::TupleLit, tuple_elements.len().uindex() + additional_len_for_first_element, extra_pointer.0)
                } else {
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    parenthesis_expression
                }
            },
            TokenKind::Undefined => self.push_node(NodeTag::Undefined, U_NONE, U_NONE),
            TokenKind::Garbage => self.push_node(NodeTag::Garbage, U_NONE, U_NONE),
            TokenKind::Self_ => self.push_node(NodeTag::Self_, U_NONE, U_NONE),
            TokenKind::Continue => self.push_node(NodeTag::Continue, U_NONE, U_NONE),
            TokenKind::Break => self.push_node(NodeTag::Break, U_NONE, U_NONE),
            TokenKind::Return | TokenKind::Pass => {
                advance_after_match = false;
                self.advance();
                let return_value = if self.check_bool(TokenKind::Semicolon) {
                    U_NONE
                } else {
                    self.parse_expression()?.0
                };
                let tag = if token_kind == TokenKind::Return { NodeTag::Return } else { NodeTag::Pass };
                self.push_node(tag, return_value, U_NONE)
            },
            TokenKind::LeftBrace => { advance_after_match = false; self.parse_and_consume_block()? },
            TokenKind::If => { advance_after_match = false; self.parse_if_expression()? },
            TokenKind::While => { advance_after_match = false; self.parse_while_expression()? },
            TokenKind::For => { advance_after_match = false; self.parse_for_expression()? },
            TokenKind::Do => { advance_after_match = false; self.parse_do_while_expression()? },
            TokenKind::Match => { advance_after_match = false; self.parse_match_expression()? },
            TokenKind::Func => { advance_after_match = false; self.parse_func_decleration_expression()? },
            _ => {
                self.error_recovery(ParseError::ExpectedToken(TokenKind::Identifier));
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
            NodeTag::PanicIfErrOrNone |
            NodeTag::BubbleIfErrOrNone => self.push_node(operator, operand.0, 0).some(),
            NodeTag::FuncCall => {
                let args = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                let extra_index = self.push_one_extra(args.len().uindex());
                self.push_extra(args.uindex_slice());
                self.push_node(NodeTag::FuncCall, operand.0, extra_index.0).some()
            },
            NodeTag::Index => {
                let safe = !self.if_matches_then_consume_bool(TokenKind::Bang);
                let index = self.parse_expression()?;
                self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket)?;
                let extra = self.push_one_extra(safe.uindex());
                self.push_one_extra(index.0);
                self.push_node(NodeTag::Index, operand.0, extra.0).some()
            }
            _ => unreachable!(),
        }
    }

    pub fn parse_and_consume_block(&mut self) -> Option<NodeIndex> {
        // debug_assert!(self.check_bool(TokenKind::LeftBrace));
        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock)?;
        let mut statements = Vec::new();
        while !self.is_at_end() && !self.check_bool(TokenKind::RightBrace) {
            if let Some(statment) = self.parse_statement() {
                statements.push(statment);
            }
        };

        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace)?;
        self.push_block(statements).some()
    }

    pub fn parse_block_or_semicolon_terminated_expression(&mut self) -> Option<NodeIndex> {
        let is_a_block = self.check_bool(TokenKind::LeftBrace);
        let block_or_expression = self.parse_expression();

        if !is_a_block {
            self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
        }

        block_or_expression
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
            TokenKind::Identifier => self.push_node(NodeTag::NamedType, current_token.index.0, 0),
            TokenKind::Union => {
                advance_after_match = false;
                self.advance();
                self.must_consume(TokenKind::LeftParen, ParseError::MissingLeftParen)?;
                let types = self.parse_comma_seperated_nodes(TokenKind::RightParen, 
                    |p| p.parse_type_decleration());
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                let extra = self.push_extra(types.uindex_slice());
                self.push_node(NodeTag::Union, types.len().uindex(), extra.0)
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
                        self.push_node(NodeTag::DynArrayType, type_.0, 0)
                    } else {
                        let size = self.parse_expression()?;
                        self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket)?;
                        self.push_node(NodeTag::FixedArrayType, type_.0, size.0)
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
            NodeTag::MemberAccess | NodeTag::Colon => {
                if self.if_matches_then_consume_bool(TokenKind::LeftParen) {
                    let args  = self.parse_comma_seperated_expressions(TokenKind::RightParen);
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    let is_mutable_self = operator == NodeTag::Colon;
                    let extra_pointer = self.push_one_extra(child.0);
                    self.push_one_extra(is_mutable_self.uindex());
                    self.push_one_extra(args.len().uindex());
                    self.push_extra(args.uindex_slice());
                    self.push_node(NodeTag::MethodCall, parent.0, extra_pointer.0).some()
                } else {
                    self.push_node(operator, parent.0, child.0).some()
                }
            }
            _ => self.push_node(operator, parent.0, child.0).some()
        }
    }

    fn push_block(&mut self, statements: Vec<NodeIndex>) -> NodeIndex {
        let block_size = statements.len().uindex();
        let extra_ptr = self.push_extra(statements.uindex_slice());
        self.push_node(NodeTag::Block, block_size, extra_ptr.0)
    }

    fn parse_identifier_or_destructure(&mut self) -> Option<NodeIndex> {
        let token = self.peek_current();
        match token.kind {
            TokenKind::Mut => {
                self.advance();
                let identifier = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                self.push_node(NodeTag::IdentifierBinding, identifier.index.0, 1).some()
            }
            TokenKind::Identifier => { 
                self.advance();
                self.push_node(NodeTag::IdentifierBinding, token.index.0, 0).some()
            },
            TokenKind::LeftParen => {
                self.advance();
                let elements = self.parse_comma_seperated_nodes(TokenKind::RightParen, |p| p.parse_identifier_or_destructure());
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                let extra_pointer = self.push_extra(elements.uindex_slice());
                self.push_node(NodeTag::TupleDestructure, elements.len().uindex(), extra_pointer.0).some()
            },
            TokenKind::LeftBracket => {
                self.advance();
                let elements = self.parse_comma_seperated_nodes(TokenKind::RightBracket, |p| p.parse_identifier_or_destructure());
                self.must_consume(TokenKind::RightBracket, ParseError::MissingRightBracket)?;
                let extra_pointer = self.push_extra(elements.uindex_slice());
                self.push_node(NodeTag::ArrayDestructure, elements.len().uindex(), extra_pointer.0).some()
            },
            TokenKind::LeftBrace => {
                self.advance();
                let binding_list = self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
                    let is_mut = p.if_matches_then_consume_bool(TokenKind::Mut);
                    let field_name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                    let binding = if p.if_matches_then_consume_bool(TokenKind::Colon) {
                        if is_mut {
                            p.error_recovery(ParseError::StructMutBinding);
                        };
                        // let field_name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                        // p.push_node(NodeTag::IdentifierBinding, field_name.index.0, is_mut.uindex())
                        p.parse_identifier_or_destructure()?
                    } else {
                        p.push_node(NodeTag::IdentifierBinding, field_name.index.0, is_mut.uindex())
                    };
                    p.push_node(NodeTag::StructDestructureBinding, field_name.index.0, binding.0).some()
                });
                self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace);
                let extra_pointer = self.push_extra(binding_list.uindex_slice());
                self.push_node(NodeTag::StructDestructure, binding_list.len().uindex(), extra_pointer.0).some()
            },
            _ => {
                self.error_recovery(ParseError::MissingIdentifier);
                None
            }
        }
    }
}

//Complicated statements or expressions
impl<'a> Parser<'a> {
    fn parse_let_statement(&mut self) -> Option<NodeIndex> {
        self.advance();
        let binding = self.parse_identifier_or_destructure()?;
        let let_type = if self.if_matches_then_consume_bool(TokenKind::Colon) {
            self.parse_type_decleration()
        } else { 
            None
        };
        self.must_consume(TokenKind::Equals, ParseError::ExpectedToken(TokenKind::Equals))?;
        let assignment = self.parse_expression()?;
        self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
        
        let extra_pointer = self.push_one_extra(let_type.option());
        self.push_one_extra(assignment.0);
        self.push_node(NodeTag::Let, binding.0, extra_pointer.0).some()
    }

    fn parse_if_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen))?;
        let condition = self.parse_expression()?;
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
        let then = self.parse_block_or_semicolon_terminated_expression()?;
        let else_ = if self.if_matches_then_consume_bool(TokenKind::Else) {
            // self.parse_block_or_semicolon_terminated_expression()?.some()
            if self.check_bool(TokenKind::If) {
                self.parse_if_expression()
            } else {
                self.parse_block_or_semicolon_terminated_expression()
            }
        } else {
            None
        };
        let extra_index = self.push_one_extra(then.0);
        self.push_one_extra(else_.option());
        self.push_node(NodeTag::If, condition.0, extra_index.0).some()
    }

    fn parse_match_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen));
        let target = self.parse_expression()?;
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen);
        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock);
        let arms = self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
            let left = p.parse_expression()?.0;
            p.must_consume(TokenKind::FatArrow, ParseError::ExpectedToken(TokenKind::FatArrow))?;
            let right = p.parse_expression()?.0;
            p.push_node(NodeTag::MatchArms, left, right).some()
        });
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace);
        let extra_index = self.push_one_extra(arms.len().uindex());
        self.push_extra(arms.uindex_slice());
        self.push_node(NodeTag::Match, target.0, extra_index.0).some()
    }

    fn parse_while_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        self.must_consume(TokenKind::LeftParen, ParseError::MissingLeftParen)?;
        let condition = self.parse_expression()?;
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
        let block = self.parse_and_consume_block()?;

        self.push_node(NodeTag::While, condition.0, block.0).some()
    }

    fn parse_do_while_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        let block = self.parse_block_or_semicolon_terminated_expression()?;
        self.must_consume(TokenKind::While, ParseError::ExpectedToken(TokenKind::While))?;
        self.must_consume(TokenKind::LeftParen, ParseError::MissingLeftParen)?;
        let condition = self.parse_expression()?;
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;

        self.push_node(NodeTag::DoWhile, condition.0, block.0).some()
    }

    fn parse_for_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        self.must_consume(TokenKind::LeftParen, ParseError::MissingLeftParen)?;
        let binding = self.parse_identifier_or_destructure()?;
        self.must_consume(TokenKind::In, ParseError::ExpectedToken(TokenKind::In))?;
        let iterator = self.parse_expression()?;
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
        let block = self.parse_block_or_semicolon_terminated_expression()?;

        let extra_pointer = self.push_one_extra(iterator.0);
        self.push_one_extra(block.0);
        self.push_node(NodeTag::For, binding.0, extra_pointer.0).some()
    }

    fn parse_func_decleration_expression(&mut self) -> Option<NodeIndex> {

        // self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen))?;
        // let params = self.parse_comma_seperated_nodes(TokenKind::RightParen, |p| {
        //     p.parse_expression()
        // });
        // self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
        // let return_type = if self.if_matches_then_consume_bool(TokenKind::SkinnyArrow) {
        //     self.parse_type_decleration()?.some()
        // } else {
        //     None
        // };
        // let extra_pointer = self.push_one_extra(return_type.option());
        // self.push_extra(param_types.uindex_slice());
        //
        // self.push_node(NodeTag::FuncExpressionDecl, param_types.len().uindex(), extra_pointer.0).some()
        todo!()
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
        TokenRef { token, index: TokenIndex(index as UIndex), kind: token.kind }
    }

    fn advance(&mut self) -> TokenRef<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }

        // &self.tokens[self.current - 1]
        let index = self.current - 1;
        let token = &self.tokens[index];
        TokenRef { token, index: TokenIndex(index as UIndex), kind: token.kind }
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

    //FIXME: (#3) Error recovery forced parser to get stuck in a loop
    //error: during the testing of if_expression_with_else_if_condition() in expressions.rs
    //an error occured when I accidentally mistyped the input "if condition) foo();"
    //which triggered a missing left parenthisis error, this caused the program to hang.
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
            TokenKind::DotDot => NodeTag::ExclusiveRange,
            TokenKind::DotDotEquals => NodeTag::InclusiveRange,
            TokenKind::Caret => NodeTag::Deref,
            TokenKind::Bang => NodeTag::PanicIfErrOrNone,
            TokenKind::Question => NodeTag::BubbleIfErrOrNone,
            TokenKind::Dot => NodeTag::MemberAccess,
            TokenKind::Colon => NodeTag::Colon,
            TokenKind::QuestionQuestionDot => NodeTag::UndefinedChainingAccess,
            TokenKind::LeftBracket => NodeTag::Index,
            TokenKind::QuestionQuestion => NodeTag::UndefinedCoalescing,
            TokenKind::As => NodeTag::As,
            TokenKind::Equals => NodeTag::Assignment,
            TokenKind::PlusEquals => NodeTag::PlusAssignment,
            TokenKind::MinusEquals => NodeTag::MinusAssignment,
            TokenKind::StarEquals => NodeTag::MultiplyAssignment,
            TokenKind::SlashEquals => NodeTag::DivideAssignment,
            TokenKind::PercentEquals => NodeTag::ModuloAssignment,
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

    fn parse_comma_seperated_nodes(
        &mut self, 
        closing_delimiter: TokenKind, 
        callback: impl Fn(&mut Parser) -> Option<NodeIndex>
    ) -> Vec<NodeIndex> {
        let mut nodes = Vec::new();
        while !self.is_at_end() && !self.check_bool(closing_delimiter) {
            if let Some(node) = callback(self) {
                nodes.push(node);
            };

            if !self.if_matches_then_consume_bool(TokenKind::Comma) {break;};
        }

        nodes
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
