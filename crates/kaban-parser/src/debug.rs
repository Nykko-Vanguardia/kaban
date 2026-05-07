use std::{fmt::{Debug, Result}, os::macos::raw::stat};
use kaban_core::SourceIndex;
use kaban_lexer::{Token, TokenPrinter, token::TokenKind};

use crate::{Parser, ast::{NodeIndex, NodeTag, TokenIndex}, parser::AST};

pub struct NodePrinter<'a> {
    ast: &'a AST<'a>,
    index: NodeIndex,
}

impl<'a> Debug for NodePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let tag = self.ast.get_tag(self.index);
        let (left, right) = self.ast.get_left_right(self.index);

        match tag {
            NodeTag::IntLit => self.write_token(f, left),
            NodeTag::Add => {
                f.debug_struct("Add")
                    .field("left", &self.child(NodeIndex(left)))
                    .field("right", &self.child(NodeIndex(right)))
                    .finish()
            }
            NodeTag::Block => {
                let statements = self.ast.get_extra_from_count(left, right);
                let statements = self.children(statements);

                f.debug_list()
                    .entries(statements.iter())
                    .finish()
            }
            _ => todo!()
        }
    }
}

impl<'a> NodePrinter<'a> {
    fn child(&self, index: NodeIndex) -> Self {
        Self {
            ast: self.ast,
            index,
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

    fn get_token(&self, index: SourceIndex) -> TokenPrinter<'a> {
        let token = self.ast.get_token(TokenIndex(index));
        token.to_debugger(self.ast.source)
    }

    fn write_token(&self, f: &mut std::fmt::Formatter<'_>, index: SourceIndex) -> std::fmt::Result {
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
