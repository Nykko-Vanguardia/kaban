use std::fmt::{Debug, Result};
use kaban_core::{ToBool, UIndex};
use kaban_lexer::{TokenPrinter};

use crate::{ast::AST, node::{NodeIndex, NodeTag, ToWrapper, TokenIndex, UIndexVec}};

pub struct NodePrinter<'a> {
    ast: &'a AST<'a>,
    index: NodeIndex,
    skip_expression_statement_indent: bool,
}

impl<'a> Debug for NodePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let tag = self.ast.get_tag(self.index);
        let (left, right) = self.ast.get_left_right(self.index);
        let index = self.index;

        match tag {
            NodeTag::Self_ => write!(f, "{:?}", tag),
            t if t.is_token_leaf() => self.write_token(f, left),
            NodeTag::BoolLit => { write!(f, "{}", left.bool()) },
            NodeTag::ExpressionStatement => {
                if self.skip_expression_statement_indent {
                    write!(f, "{:#?}", &self.child(left))
                } else {
                    f.debug_tuple("ExpressionStatement").field(&self.child(left)).finish()
                }
            },
            t if t.is_binary_expression() => {
                f.debug_struct(format!("{:?}", t).as_str())
                    .field("left", &self.child(left))
                    .field("right", &self.child(right))
                    .finish()
            },
            NodeTag::ArrayLit |
            NodeTag::Block |
            NodeTag::Union |
            NodeTag::TupleDestructure |
            NodeTag::StructDestructure |
            NodeTag::TupleLit => {
                let general_list = self.ast.view_general_list(index);
                write!(f, "{:?} ", tag)?;
                f.debug_list()
                    .entries(self.children(general_list.indices).iter())
                    .finish()
            },
            NodeTag::Index => {
                let index = self.ast.view_index(index);
                f.debug_struct("Index")
                    .field("callee", &self.child(index.callee.0))
                    .field("index", &self.child(index.index_by.0))
                    .field("is_safe", &index.is_safe_index)
                    .finish()
            },
            NodeTag::FuncCall => {
                let func_call = self.ast.view_func_call(index);
                f.debug_struct("FuncCall")
                    .field("callee", &self.child(left))
                    .field("args", &self.children(func_call.args.uindex_slice()))
                    .finish()
            }
            NodeTag::New | NodeTag::Destruct => todo!(),
            t if t.is_postfix() || t.is_prefix() => {
                f.debug_tuple(format!("{:?}", t).as_str())
                    .field(&self.child(left))
                    .finish()
            }
            NodeTag::FixedArrayType => {
                f.debug_struct("FixedArrayType")
                    .field("type", &self.child(left))
                    .field("size", &self.child(right))
                    .finish()

            },
            NodeTag::NamedType => {
                f.debug_tuple("NamedType")
                    .field(&self.get_token(left))
                    .finish()
            },
            t if t.is_simple_modifier_type() => {
                f.debug_tuple(format!("{:?}", t).as_str())
                    .field(&self.child(left))
                    .finish()
            },
            NodeTag::MethodCall => {
                let method = self.ast.view_method_call(index);
                f.debug_struct("MethodCall")
                    .field("callee", &self.child(method.callee.0))
                    .field("method name", &self.child(method.method_name.0))
                    .field("is mutable", &method.is_self_mut)
                    .field("args", &self.children(method.args.uindex_slice()))
                    .finish()
            }
            t if t.is_type() => write!(f, "Type({:?})", t),
            NodeTag::If => {
                let if_ = self.ast.view_if_expression(index);
                if if_.else_.is_some() {
                    f.debug_struct("IfExpression")
                        .field("condition", &self.child(if_.condition.0))
                        .field("then", &self.child(if_.then.0))
                        .field("else", &self.child(if_.else_.unwrap()))
                        .finish()
                } else {
                    f.debug_struct("IfExpression")
                        .field("condition", &self.child(if_.condition.0))
                        .field("then", &self.child(if_.then.0))
                        .finish()
                }
            },
            NodeTag::Match => {
                let match_ = self.ast.view_match_expression(index);
                f.debug_struct("Match")
                    .field("target", &self.child(match_.target.0))
                    .field("arms", &self.children(match_.arms.uindex_slice()))
                    .finish()
            },
            NodeTag::MatchArms => {
                f.debug_struct("Arm")
                    .field("left", &self.child(left))
                    .field("right", &self.child(right))
                    .finish()
            },
            NodeTag::DoWhile | NodeTag::While => {
                f.debug_struct(format!("{:?}", tag).as_str())
                    .field("block", &self.child(right))
                    .field("condition", &self.child(left))
                    .finish()
            },
            NodeTag::For => {
                let for_loop = self.ast.view_for_expression(index);
                f.debug_struct("For")
                    .field("binding", &self.child(for_loop.binding.0))
                    .field("iterator", &self.child(for_loop.iterator.0))
                    .field("block", &self.child(for_loop.block.0))
                    .finish()
            },
            NodeTag::IdentifierBinding =>
                f.debug_struct("IdentifierBinding")
                .field("name", &self.get_token(left))
                .field("mutable", &right.bool())
                .finish(),
            NodeTag::StructDestructureBinding =>
                f.debug_struct("StructDestructureBinding")
                .field("field_name", &self.get_token(left))
                .field("binding", &self.child(right))
                .finish(),
            NodeTag::Let => {
                let let_ = self.ast.view_let_statement(index);
                if let_.type_.is_some() {
                    f.debug_struct("Let")
                        .field("name", &self.child(let_.name.0))
                        .field("type", &self.child(let_.type_.unwrap()))
                        .field("assignment", &self.child(let_.assignment.0))
                        .finish()
                } else {
                    f.debug_struct("Let")
                        .field("name", &self.child(let_.name.0))
                        .field("assignment", &self.child(let_.assignment.0))
                        .finish()
                }
            },
            NodeTag::Continue | NodeTag::Break => write!(f, "{:?}", tag),
            NodeTag::Return | NodeTag::Pass => {
                if left.uoption().is_some() {
                    f.debug_tuple(format!("{:?}", tag).as_str()).field(&self.child(left)).finish()
                } else {
                    f.debug_tuple(format!("{:?}", tag).as_str()).finish()
                }
            }

            _ => todo!("NOT IMPLEMENTED YET: {:?}", tag)
        }
    }
}

impl<'a> NodePrinter<'a> {
    fn child(&self, index: UIndex) -> Self {
        Self {
            ast: self.ast,
            index: NodeIndex(index),
            skip_expression_statement_indent: self.skip_expression_statement_indent,
        }
    }

    fn children(&self, indices: &[u32]) -> Vec<Self> {
        let mut children = Vec::new();
        for index in indices.iter().copied()  {
            children.push(Self {
                ast: self.ast,
                index: NodeIndex(index),
                skip_expression_statement_indent: self.skip_expression_statement_indent,
            });
        };
        children
    }

    fn get_token(&self, index: UIndex) -> TokenPrinter<'a> {
        let token = self.ast.get_token(TokenIndex(index));
        token.to_debugger(self.ast.get_source())
    }

    fn write_token(&self, f: &mut std::fmt::Formatter<'_>, index: UIndex) -> std::fmt::Result {
        write!(f, "{:?}", self.get_token(index))
    }
}

impl<'a> AST<'a> {
    pub fn to_debugger(&'a self) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: self.root,
            skip_expression_statement_indent: false,
        }
    }

    pub fn to_debugger_options(&'a self, skip_expression_statement: bool) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: self.root,
            skip_expression_statement_indent: skip_expression_statement,
        }
    }

    pub fn create_debugger(&'a self, node: NodeIndex) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: node,
            skip_expression_statement_indent: false,
        }
    }
}
