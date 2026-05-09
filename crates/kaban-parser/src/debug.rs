use std::fmt::{Debug, Result};
use kaban_core::{ToBool, UIndex};
use kaban_lexer::{TokenPrinter};

use crate::{ast::AST, node::{NodeIndex, NodeTag, TokenIndex, UIndexVec}};

pub struct NodePrinter<'a> {
    ast: &'a AST<'a>,
    index: NodeIndex,
}

impl<'a> Debug for NodePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let tag = self.ast.get_tag(self.index);
        let (left, right) = self.ast.get_left_right(self.index);
        let index = self.index;

        match tag {
            t if t.is_token_leaf() => self.write_token(f, left),
            NodeTag::BoolLit => { write!(f, "{}", left.bool()) },
            t if t.is_binary_expression() => {
                f.debug_struct(format!("{:?}", t).as_str())
                    .field("left", &self.child(left))
                    .field("right", &self.child(right))
                    .finish()
            },
            NodeTag::ArrayLit | NodeTag::Block | NodeTag::Union => {
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
                    .field("method name", &self.child(method.method_name.0))
                    .field("is mutable", &method.is_self_mut)
                    .field("args", &self.children(method.args.uindex_slice()))
                    .finish()
            }
            t if t.is_type() => write!(f, "Type({:?})", t),
            _ => todo!()
        }
    }
}

impl<'a> NodePrinter<'a> {
    fn child(&self, index: UIndex) -> Self {
        Self {
            ast: self.ast,
            index: NodeIndex(index),
        }
    }

    fn children(&self, indices: &[u32]) -> Vec<Self> {
        let mut children = Vec::new();
        for index in indices.iter().copied()  {
            children.push(Self {
                ast: self.ast,
                index: NodeIndex(index),
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
            index: self.root
        }
    }

    pub fn create_debugger(&'a self, node: NodeIndex) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: node
        }
    }
}
