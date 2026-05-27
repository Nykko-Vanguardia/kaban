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
        self.node_tags.clear();
        self.node_data.clear();
        self.extra.clear();

        self.tokens = tokens;
        self.source = source;
        self.current = 0;
    }

    pub fn parse_statement(&mut self) -> Option<NodeIndex> {
        let is_pub = self.if_matches_then_consume_bool(TokenKind::Pub);
        let current_token = self.peek_current();

        match current_token.kind {
            TokenKind::Let => { 
                if is_pub { self.error_recovery(ParseError::PubInLet); }
                self.parse_let_statement()
            },
            TokenKind::Const => self.parse_const_statement(is_pub),
            TokenKind::Func => self.parse_func_decleration(is_pub),
            TokenKind::Struct => self.parse_struct_decleration(is_pub),
            TokenKind::Enum => self.parse_enum_decleration(is_pub),
            TokenKind::Impl => self.parse_impl_decleration(is_pub),
            TokenKind::Interface => self.parse_interface_decleration(is_pub),
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
            TokenKind::LeftBrace
                if self.is_anonymous_struct_instantiation() => { advance_after_match = false; self.parse_struct_instantiation(None)? },
            TokenKind::LeftBrace => { advance_after_match = false; self.parse_and_consume_block()? },
            TokenKind::If => { advance_after_match = false; self.parse_if_expression()? },
            TokenKind::While => { advance_after_match = false; self.parse_while_expression()? },
            TokenKind::For => { advance_after_match = false; self.parse_for_expression()? },
            TokenKind::Do => { advance_after_match = false; self.parse_do_while_expression()? },
            TokenKind::Match => { advance_after_match = false; self.parse_match_expression()? },
            TokenKind::Func => { advance_after_match = false; self.parse_anonymous_func_decleration_expression()? },
            TokenKind::At => { advance_after_match = false; self.parse_comptime_expression()? },
            TokenKind::Enum if self.peek_next().kind == TokenKind::Dot
                => self.push_node(NodeTag::AnonymousEnumlit, U_NONE, U_NONE),
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
            // NodeTag::New | NodeTag::Destruct => {
            //     // self.advance();
            //     let method_call = self.parse_expression()?;
            //     self.push_node(prefix_unary, method_call.0, U_NONE).some()
            // },
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
            NodeTag::GenericInstantiation => {
                self.must_consume(TokenKind::Less, ParseError::ExpectedToken(TokenKind::Less))?;
                let args = self.parse_comma_seperated_nodes(TokenKind::Greater, |p| p.parse_type_decleration());
                self.must_consume(TokenKind::Greater, ParseError::MissingGreater)?;

                let extra_index = self.push_one_extra(args.len().uindex());
                self.push_extra(args.uindex_slice());
                self.push_node(NodeTag::GenericInstantiation, operand.0, extra_index.0).some()
            },
            NodeTag::StructInstantiation => self.parse_struct_instantiation(operand.some()),
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
            TokenKind::LeftParen => {
                advance_after_match = false;
                self.advance();
                let first_type_ = self.parse_type_decleration()?;
                if self.if_matches_then_consume_bool(TokenKind::Comma) {
                    let the_rest_of_the_types = self.parse_comma_seperated_nodes(TokenKind::RightParen, |p| p.parse_type_decleration());
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    const FIRST_TYPE_LEN: u32 = 1;
                    let extra_pointer = self.push_one_extra(first_type_.0);
                    self.push_extra(the_rest_of_the_types.uindex_slice());
                    self.push_node(NodeTag::TupleType, the_rest_of_the_types.len().uindex() + FIRST_TYPE_LEN, extra_pointer.0)
                } else {
                    self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                    first_type_
                }
            }
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
            TokenKind::Struct => {
                advance_after_match = false;
                self.advance();
                self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock);
                let field_declerations = self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
                    let field_name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                    p.must_consume(TokenKind::Colon, ParseError::ExpectedToken(TokenKind::Colon))?;
                    let type_ = p.parse_type_decleration()?;
                    p.push_node(NodeTag::AnonymousStructFieldDecl, field_name.index.0, type_.0).some()
                });
                self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace);
                let extra_pointer = self.push_extra(field_declerations.uindex_slice());

                self.push_node(NodeTag::AnonymousStructType, field_declerations.len().uindex(), extra_pointer.0)
            }
            TokenKind::Func => {
                advance_after_match = false;
                self.advance();
                self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen))?;
                let params = self.parse_comma_seperated_nodes(TokenKind::RightParen, |p| {
                    let is_mut = p.if_matches_then_consume_bool(TokenKind::Mut).uindex();
                    let identifier = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                    let identifier_binding = p.push_node(NodeTag::IdentifierBinding, identifier.index.0, is_mut);

                    p.must_consume(TokenKind::Colon, ParseError::MissingTypeDeclaration)?;
                    let type_ = p.parse_type_decleration()?;

                    p.push_node(NodeTag::Params, identifier_binding.0, type_.0).some()
                });
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;

                let return_type = if self.if_matches_then_consume_bool(TokenKind::SkinnyArrow) {
                    self.parse_type_decleration()?.some()
                } else {
                    None
                };
                let extra_pointer = self.push_one_extra(params.len().uindex());
                self.push_extra(params.uindex_slice());

                self.push_node(NodeTag::FuncType, return_type.to_index_or_u_none(), extra_pointer.0)
            }
            TokenKind::Enum if self.peek_next().kind == TokenKind::LeftBrace => {
                advance_after_match = false;
                self.advance();
                let enum_variant_declerations = self.parse_enum_block();
                let extra_pointer = self.push_extra(enum_variant_declerations.uindex_slice());
                self.push_node(NodeTag::AnonymousEnumType, enum_variant_declerations.len().uindex(), extra_pointer.0)
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
                TokenKind::Less => {
                    advance_after_match = false;
                    self.advance();
                    let types = self.parse_comma_seperated_nodes(TokenKind::Greater, |p| p.parse_type_decleration());
                    self.must_consume(TokenKind::Greater, ParseError::MissingGreater);
                    let extra_pointer = self.push_one_extra(types.len().uindex());
                    self.push_extra(types.uindex_slice());
                    self.push_node(NodeTag::TypeWithGenerics, type_.0, extra_pointer.0)
                }
                _ => break,
            };

            if advance_after_match {
                self.advance();
            };
        }

        Some(type_)
    }

    fn if_angle_bracket_parse_generic_declerations_else_none(&mut self) -> Option<Vec<NodeIndex>> {
        if self.if_matches_then_consume_bool(TokenKind::Less) {
            let generic_list = self.parse_comma_seperated_nodes(TokenKind::Greater, |p| {
                let name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                let constraint = if p.if_matches_then_consume_bool(TokenKind::Colon) {
                    p.parse_generic_constraint()
                } else {
                    None
                };

                p.push_node(NodeTag::GenericParam, name.index.0, constraint.to_index_or_u_none()).some()
            });
            self.must_consume(TokenKind::Greater, ParseError::MissingGreater)?;

            Some(generic_list)
        } else {
            None
        }
    }

    fn parse_generic_constraint_atom(&mut self) -> Option<NodeIndex> {
        match self.peek_current().kind {
            TokenKind::LeftParen => {
                self.advance();
                let generic = self.parse_generic_constraint()?;
                self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
                generic
            }
            TokenKind::Impl => {
                self.advance();
                let interface = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
                self.push_node(NodeTag::InterfaceConstraint, interface.index.0, U_NONE)
            }
            _ => self.parse_type_decleration()?,
        }.some()
    }

    //NOTE: FOR NOW ITS ALWAYS LEFT PRECEDENCE, I do not know if i want to add precedence of and over
    //or
    fn parse_generic_constraint(&mut self) -> Option<NodeIndex> {
        let mut left = self.parse_generic_constraint_atom()?;

        loop {
            let current = self.peek_current().kind;
            match current {
                TokenKind::Ampersand | TokenKind::Pipe => {
                    self.advance();
                    let right = self.parse_generic_constraint_atom()?;
                    let tag = match current {
                        TokenKind::Ampersand => NodeTag::AndGenericConstaint,
                        TokenKind::Pipe => NodeTag::OrGenericConstaint,
                        _ => unreachable!()
                    };
                    left = self.push_node(tag, left.0, right.0);
                }
                _ => break 
            }
        };

        left.some()
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
                } else if self.if_matches_then_consume_bool(TokenKind::At) {
                    self.parse_generic_instantiated_member_access_or_method(parent, child, operator)
                } else {
                    if operator == NodeTag::Colon {self.error_recovery(ParseError::ExpectedMethod);}
                    self.push_node(operator, parent.0, child.0).some()
                }
            }
            NodeTag::UndefinedChainingAccess |
                NodeTag::ImplAccess => self.push_node(operator, parent.0, child.0).some(),
            _ => unreachable!(),
        }

    }

    fn parse_generic_instantiated_member_access_or_method(&mut self, parent: NodeIndex, child: NodeIndex, operator: NodeTag) -> Option<NodeIndex> {
        self.must_consume(TokenKind::Less, ParseError::ExpectedToken(TokenKind::Less))?;
        let generic_args = self.parse_comma_seperated_nodes(TokenKind::Greater, |p| p.parse_type_decleration());
        self.must_consume(TokenKind::Greater, ParseError::MissingGreater)?;

        if self.if_matches_then_consume_bool(TokenKind::LeftParen) {
            let args  = self.parse_comma_seperated_expressions(TokenKind::RightParen);
            self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
            let is_mutable_self = operator == NodeTag::Colon;
            let extra_pointer = self.push_one_extra(child.0);
            self.push_one_extra(is_mutable_self.uindex());
            self.push_one_extra(args.len().uindex());
            self.push_one_extra(generic_args.len().uindex());
            self.push_extra(args.uindex_slice());
            self.push_extra(generic_args.uindex_slice());
            self.push_node(NodeTag::MethodWithGenericInstantiation, parent.0, extra_pointer.0).some()
        } else {
            if operator == NodeTag::Colon {self.error_recovery(ParseError::ExpectedMethod);}
            let member_access = self.push_node(operator, parent.0, child.0);

            let extra_index = self.push_one_extra(generic_args.len().uindex());
            self.push_extra(generic_args.uindex_slice());
            self.push_node(NodeTag::GenericInstantiation,member_access.0 , extra_index.0).some()
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
        if !self.node_tags[assignment.0.usize()].doesnt_require_semicolon() {
            self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
        }
        
        let extra_pointer = self.push_one_extra(let_type.to_index_or_u_none());
        self.push_one_extra(assignment.0);
        self.push_node(NodeTag::Let, binding.0, extra_pointer.0).some()
    }

    fn parse_const_statement(&mut self, is_pub: bool) -> Option<NodeIndex> {
        self.advance();
        let identifier = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
        self.must_consume(TokenKind::Colon, ParseError::MissingTypeDeclaration)?;
        let type_ = self.parse_type_decleration()?;

        self.must_consume(TokenKind::Equals, ParseError::ExpectedToken(TokenKind::Equals))?;
        let assignment = self.parse_expression()?;
        if !self.node_tags[assignment.0.usize()].doesnt_require_semicolon() {
            self.must_consume(TokenKind::Semicolon, ParseError::MissingSemicolon)?;
        }
        
        let extra_pointer = self.push_one_extra(is_pub.uindex());
        self.push_one_extra(type_.0);
        self.push_one_extra(assignment.0);
        self.push_node(NodeTag::Const, identifier.index.0, extra_pointer.0).some()
    }
    
    fn parse_func_decleration_or_header(&mut self, is_pub: bool) -> Option<NodeIndex> {
        self.advance();
        let name = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
        let generics = self.if_angle_bracket_parse_generic_declerations_else_none();

        self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen))?;
        let self_ = self.parse_self_param();
        let params = self.parse_comma_seperated_nodes(TokenKind::RightParen, |p| {
            let identifier_binding = p.parse_identifier_or_destructure()?;
            p.must_consume(TokenKind::Colon, ParseError::MissingTypeDeclaration)?;
            let type_ = if p.check_bool(TokenKind::Impl) {
                p.parse_generic_constraint()?
            } else {
                p.parse_type_decleration()?
            };

            p.push_node(NodeTag::Params, identifier_binding.0, type_.0).some()
        });
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
        let return_type = if self.if_matches_then_consume_bool(TokenKind::SkinnyArrow) {
            self.parse_type_decleration()?.some()
        } else {
            None
        };
        let block = if self.peek_current().kind == TokenKind::LeftBrace {
            self.parse_and_consume_block()
        } else {
            self.must_consume(TokenKind::Semicolon, ParseError::MissingBlockOrSemicolon);
            None
        };
        let has_block = block.is_some();

        let extra_pointer = self.push_one_extra(is_pub.uindex());
        self.push_one_extra(return_type.to_index_or_u_none());
        if let Some(block) = block { self.push_one_extra(block.0); }

        let add_self_to_param_len = if self_.is_some() { 1 } else { 0 };
        if let Some(generics) = generics {
            self.push_one_extra(generics.len().uindex());
            self.push_one_extra(params.len().uindex() + add_self_to_param_len);
            self.push_extra(generics.uindex_slice());
            if let Some(self_) = self_ {
                self.push_one_extra(self_.0);
            }
            self.push_extra(params.uindex_slice());
            let tag = if has_block { NodeTag::FuncDeclWithGenerics } else { NodeTag::FuncNoBodyWithGenerics };
            self.push_node(tag, name.index.0, extra_pointer.0).some()
        } else {
            self.push_one_extra(params.len().uindex() + add_self_to_param_len);
            if let Some(self_) = self_ {
                self.push_one_extra(self_.0);
            }
            self.push_extra(params.uindex_slice());
            let tag = if has_block { NodeTag::FuncDeclWithNoGenerics } else { NodeTag::FuncNoBodyWithNoGenerics };
            self.push_node(tag, name.index.0, extra_pointer.0).some()
        }
    }

    fn parse_func_decleration(&mut self, is_pub: bool) -> Option<NodeIndex> {
        let func = self.parse_func_decleration_or_header(is_pub)?;
        if self.node_tags[func.0.usize()] == NodeTag::FuncNoBodyWithNoGenerics || self.node_tags[func.0.usize()] == NodeTag::FuncNoBodyWithGenerics  {
            self.error_recovery(ParseError::MissingBlock);
            return None;
        }

        func.some()
    }

    fn parse_self_param(&mut self) -> Option<NodeIndex> {
        //NOTE: MIGHT REMOVE THIS
        let mut_self = self.check_bool(TokenKind::Mut) && self.peek_next().kind == TokenKind::Self_;
        if self.if_matches_then_consume_bool(TokenKind::Self_) || mut_self {
            //NOTE: MIGHT REMOVE THIS
            if mut_self {
                self.advance();
                self.advance();
            }
            let current = self.peek_current();
            let self_ = match current.kind {
                TokenKind::Ampersand |
                TokenKind::AmpersandMut |
                TokenKind::Star => {
                    self.advance();
                    self.push_node(NodeTag::SelfParam, current.index.0, mut_self.uindex()).some()
                }
                //NOTE: MIGHT REMOVE THIS
                _ => self.push_node(NodeTag::SelfParam, U_NONE, mut_self.uindex()).some()
            };
            if !self.check_bool(TokenKind::RightParen) {
                self.must_consume(TokenKind::Comma, ParseError::ExpectedToken(TokenKind::Comma))?;
            }
            self_
        } else {
            None
        }
    }

    fn parse_struct_decleration(&mut self, is_pub: bool) -> Option<NodeIndex> {
        self.advance();
        let name = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
        let generics = self.if_angle_bracket_parse_generic_declerations_else_none();

        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock);
        let field_declerations = self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
            let is_pub = p.if_matches_then_consume_bool(TokenKind::Pub);
            let field_name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
            p.must_consume(TokenKind::Colon, ParseError::ExpectedToken(TokenKind::Colon))?;
            let type_ = p.parse_type_decleration()?;
            let extra_pointer = p.push_one_extra(is_pub.uindex());
            p.push_one_extra(type_.0);
            p.push_node(NodeTag::StructFieldDecleration, field_name.index.0, extra_pointer.0).some()
        });
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace);
        let extra_pointer = self.push_one_extra(is_pub.uindex());
        if let Some(generics) = generics {
            self.push_one_extra(generics.len().uindex());
            self.push_one_extra(field_declerations.len().uindex());
            self.push_extra(generics.uindex_slice());
            self.push_extra(field_declerations.uindex_slice());
            self.push_node(NodeTag::StructDeclWithGeneric, name.index.0, extra_pointer.0).some()
        } else {
            self.push_one_extra(field_declerations.len().uindex());
            self.push_extra(field_declerations.uindex_slice());
            self.push_node(NodeTag::StructDeclWithNoGeneric, name.index.0, extra_pointer.0).some()
        }
    }

    fn parse_enum_decleration(&mut self, is_pub: bool) -> Option<NodeIndex> {
        self.advance();
        let name = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
        let generics = self.if_angle_bracket_parse_generic_declerations_else_none();

        let enum_variant_declerations = self.parse_enum_block();

        let extra_pointer = self.push_one_extra(is_pub.uindex());
        if let Some(generics) = generics {
            self.push_one_extra(generics.len().uindex());
            self.push_one_extra(enum_variant_declerations.len().uindex());
            self.push_extra(generics.uindex_slice());
            self.push_extra(enum_variant_declerations.uindex_slice());
            self.push_node(NodeTag::EnumDeclWithGeneric, name.index.0, extra_pointer.0).some()
        } else {
            self.push_one_extra(enum_variant_declerations.len().uindex());
            self.push_extra(enum_variant_declerations.uindex_slice());
            self.push_node(NodeTag::EnumDeclWithNoGeneric, name.index.0, extra_pointer.0).some()
        }
    }

    fn parse_enum_block(&mut self) -> Vec<NodeIndex> {
        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock);
        let enum_variants = self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
            let variant_name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
            let type_ = if p.if_matches_then_consume_bool(TokenKind::Colon) {
                p.parse_type_decleration()
            } else {
                None
            };

            p.push_node(NodeTag::EnumVariantDecl, variant_name.index.0, type_.to_index_or_u_none()).some()
        });
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace);
        enum_variants
    }

    fn parse_impl_decleration(&mut self, is_pub: bool) -> Option<NodeIndex> {
        self.advance();
        let type_or_interface = self.parse_type_decleration()?;
        let (type_, interface) = if self.if_matches_then_consume_bool(TokenKind::For) {
            let type_ = self.parse_type_decleration()?;
            (type_, type_or_interface.some())
        } else {
            (type_or_interface, None)
        };
        self.must_consume(TokenKind::ColonColon, ParseError::ExpectedToken(TokenKind::ColonColon))?;
        let impl_name = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
        let generics = self.if_angle_bracket_parse_generic_declerations_else_none();

        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock)?;
        let mut statements = Vec::new();
        while !self.is_at_end() && self.peek_current().kind != TokenKind::RightBrace {
            let is_inside_pub = self.if_matches_then_consume_bool(TokenKind::Pub);
            let statement = match self.peek_current().kind {
                TokenKind::Func => self.parse_func_decleration_or_header(is_inside_pub),
                TokenKind::Const => self.parse_const_statement(is_inside_pub),
                TokenKind::At => self.parse_comptime_expression(),
                _ => {
                    self.error_recovery(ParseError::InvalidImplItem);
                    return None;
                }
            };
            statements.push(statement?);
        }
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace)?;

        let extra_pointer = self.push_one_extra(is_pub.uindex());
        self.push_one_extra(type_.0);
        if let Some(interface) = interface { self.push_one_extra(interface.0); };

        if let Some(generics) = generics {
            self.push_one_extra(generics.len().uindex());
            self.push_one_extra(statements.len().uindex());
            self.push_extra(generics.uindex_slice());
            self.push_extra(statements.uindex_slice());
            let tag = if interface.is_some() { NodeTag::ImplForDeclWithGeneric } else { NodeTag::ImplDeclWithGeneric };
            self.push_node(tag, impl_name.index.0, extra_pointer.0).some()
        } else {
            self.push_one_extra(statements.len().uindex());
            self.push_extra(statements.uindex_slice());
            let tag = if interface.is_some() { NodeTag::ImplForDeclWithNoGeneric } else { NodeTag::ImplDeclWithNoGeneric };
            self.push_node(tag, impl_name.index.0, extra_pointer.0).some()
        }
    }
    fn parse_interface_decleration(&mut self, is_pub: bool) -> Option<NodeIndex> {
        self.advance();
        let name = self.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
        let generics = self.if_angle_bracket_parse_generic_declerations_else_none();
        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock)?;
        let shape = if self.if_identifier_says_shape_consume_bool() {
            self.must_consume(TokenKind::Colon, ParseError::MissingTypeDeclaration);
            self.parse_generic_constraint()
        } else {
            None
        };

        let mut statements = Vec::new();
        while !self.is_at_end() && self.peek_current().kind != TokenKind::RightBrace {
            let is_inside_pub = self.if_matches_then_consume_bool(TokenKind::Pub);
            let statement = match self.peek_current().kind {
                TokenKind::Func => self.parse_func_decleration_or_header(is_inside_pub),
                // TokenKind::Const => self.parse_const_statement(is_inside_pub),
                TokenKind::At => self.parse_comptime_expression(),
                _ => {
                    self.error_recovery(ParseError::InvalidImplItem);
                    return None;
                }
            };
            statements.push(statement?);
        }
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace)?;

        let extra_pointer = self.push_one_extra(is_pub.uindex());
        self.push_one_extra(shape.to_index_or_u_none());
        if let Some(generics) = generics {
            self.push_one_extra(generics.len().uindex());
            self.push_one_extra(statements.len().uindex());
            self.push_extra(generics.uindex_slice());
            self.push_extra(statements.uindex_slice());
            self.push_node(NodeTag::InterfaceDeclWithGenerics, name.index.0, extra_pointer.0).some()
        } else {
            self.push_one_extra(statements.len().uindex());
            self.push_extra(statements.uindex_slice());
            self.push_node(NodeTag::InterfaceDeclWithNoGenerics, name.index.0, extra_pointer.0).some()
        }
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
        self.push_one_extra(else_.to_index_or_u_none());
        self.push_node(NodeTag::If, condition.0, extra_index.0).some()
    }

    fn parse_match_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen));
        let target = self.parse_expression()?;
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen);
        self.must_consume(TokenKind::LeftBrace, ParseError::MissingBlock);
        let arms = self.parse_match_arms();
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace);
        let extra_index = self.push_one_extra(arms.len().uindex());
        self.push_extra(arms.uindex_slice());
        self.push_node(NodeTag::Match, target.0, extra_index.0).some()
    }

    fn parse_match_arms(&mut self) -> Vec<NodeIndex> {
        self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
            let left = p.parse_expression()?;
            let left = if p.check_bool(TokenKind::Pipe) {
                let mut match_targets = Vec::new();
                while !p.is_at_end() && p.if_matches_then_consume_bool(TokenKind::Pipe) {
                    match_targets.push(p.parse_expression()?);
                };
                let extra_pointer = p.push_one_extra(left.0);
                p.push_extra(match_targets.uindex_slice());
                const ORIGINAL_LEFT: UIndex = 1;
                let len = match_targets.len().uindex() + ORIGINAL_LEFT;
                p.push_node(NodeTag::MultipleMatchTargets, len, extra_pointer.0)
            } else {
                left
            };
            p.must_consume(TokenKind::FatArrow, ParseError::ExpectedToken(TokenKind::FatArrow))?;
            let right = p.parse_expression()?;
            p.push_node(NodeTag::MatchArms, left.0, right.0).some()
        })
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

    fn parse_struct_instantiation(&mut self, struct_name: Option<NodeIndex>) -> Option<NodeIndex> {
        //this is because if struct is parsed within the operator loop AKA not in atom or prefix,
        //the operator { is consumed. If its an atom its not
        if struct_name.is_none() {
            self.advance();
        }
        let field_instantiations = self.parse_comma_seperated_nodes(TokenKind::RightBrace, |p| {
            let field_name = p.must_consume(TokenKind::Identifier, ParseError::MissingIdentifier)?;
            let assignment = if p.if_matches_then_consume_bool(TokenKind::Colon) {
                p.parse_expression()?
            } else {
                p.push_node(NodeTag::Identifier, field_name.index.0, U_NONE)
            };

            p.push_node(NodeTag::StructFieldInstantiation, field_name.index.0, assignment.0).some()
        });
        self.must_consume(TokenKind::RightBrace, ParseError::MissingRightBrace)?;

        let extra_pointer = self.push_one_extra(field_instantiations.len().uindex());
        self.push_extra(field_instantiations.uindex_slice());
        self.push_node(NodeTag::StructInstantiation, struct_name.to_index_or_u_none(), extra_pointer.0).some()
    }

    fn parse_anonymous_func_decleration_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        self.must_consume(TokenKind::LeftParen, ParseError::ExpectedToken(TokenKind::LeftParen))?;
        let params = self.parse_comma_seperated_nodes(TokenKind::RightParen, |p| {
            let identifier_binding = p.parse_identifier_or_destructure()?;
            let type_ = if p.if_matches_then_consume_bool(TokenKind::Colon) {
                p.parse_type_decleration()
            } else {
                None
            };

            p.push_node(NodeTag::Params, identifier_binding.0, type_.to_index_or_u_none()).some()
        });
        self.must_consume(TokenKind::RightParen, ParseError::MissingRightParen)?;
        let return_type = if self.if_matches_then_consume_bool(TokenKind::SkinnyArrow) {
            self.parse_type_decleration()?.some()
        } else {
            None
        };
        let block = self.parse_expression()?; //Still deciding if i force a block
        let extra_pointer = self.push_one_extra(return_type.to_index_or_u_none());
        self.push_one_extra(params.len().uindex());
        self.push_extra(params.uindex_slice());

        self.push_node(NodeTag::AnonymousFuncDecl, block.0, extra_pointer.0).some()
    }

    fn parse_comptime_expression(&mut self) -> Option<NodeIndex> {
        self.advance();
        let expression = self.parse_block_or_semicolon_terminated_expression()?;
        self.push_node(NodeTag::CompTimeExpression, expression.0, U_NONE).some()
    }
}

//helper
impl<'a> Parser<'a> {
    fn peek_current(&self) -> TokenRef<'a> {
        self.peek_offset(0)
    }

    fn peek_next(&self) -> TokenRef<'a> {
        self.peek_offset(1)
    }

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
            TokenKind::ColonColon => NodeTag::ImplAccess,
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
            TokenKind::At => NodeTag::GenericInstantiation,
            TokenKind::LeftBrace => NodeTag::StructInstantiation,
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
            TokenKind::Ampersand => NodeTag::ReferenceOf,
            TokenKind::AmpersandMut => NodeTag::MutReferenceOf,
            TokenKind::Star => NodeTag::OwnershipOf,
            // TokenKind::New => NodeTag::New,
            // TokenKind::Destruct => NodeTag::Destruct,
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

    fn is_anonymous_struct_instantiation(&self) -> bool {
        debug_assert!(self.peek_current().kind == TokenKind::LeftBrace);
        let next = self.peek_next().kind;
        let third = self.peek_offset(2).kind;

        next == TokenKind::Identifier &&
            (third == TokenKind::Colon ||
             // third == TokenKind::RightBrace || //decided to remove this, do {x,} for one field
             third == TokenKind::Comma)
    }

    fn if_identifier_says_shape_consume_bool(&mut self) -> bool {
        let current = self.peek_current();
        if current.kind == TokenKind::Identifier && self.source.matches(current.span(), "shape") {
            self.advance();
            true
        } else {
            false
        }
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
