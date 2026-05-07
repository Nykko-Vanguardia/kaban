use kaban_core::{SourceIndex, ToUsize};
use kaban_lexer::{Token};

use crate::{ast::{DataIndex, NodeData, NodeIndex, NodeTag, TokenIndex}, parser::AST};

impl<'a> AST<'a> {
    #[inline(always)]
    pub fn get_token(&self, index: TokenIndex) -> &'a Token {
        &self.tokens[index.0.usize()]
    }

    #[inline(always)]
    pub fn get_tag(&'a self, index: NodeIndex) -> NodeTag {
        self.node_tags[index.0.usize()]
    }

    #[inline(always)]
    pub fn get_data(&self, index: NodeIndex) -> NodeData {
        self.node_data[index.0.usize()]
    }


    #[inline(always)]
    pub fn get_left_right(&self, index: NodeIndex) -> (SourceIndex, SourceIndex) {
        let data = self.get_data(index);
        (data.left, data.right)
    }

    pub fn get_extra_span(&self, start: SourceIndex, end: SourceIndex) -> &[SourceIndex] {
        &self.extra[start.usize()..end.usize()]
    }

    pub fn get_extra_from_count(&self, count: SourceIndex, start: SourceIndex) -> &[SourceIndex] {
        &self.extra[start.usize()..(start + count).usize()]
    }
}

// use kaban_core::SourceSpan;
// use crate::ast::{NodeIndex, Type};
// use crate::operator;
//
// pub struct Block {
//     statements: Vec<NodeIndex>,
//     value: Option<NodeIndex>,
// }
//
// pub struct If {
//     condition: NodeIndex,
//     then_block: NodeIndex,
//     else_block: Option<NodeIndex>,
// }
//
// pub struct MatchView {
//     subject: NodeIndex,
//     arms: Vec<NodeIndex>,
// }
//
// pub struct ArithmeticOperationView {
//     left: NodeIndex,
//     right: NodeIndex,
//     operator: operator::Arithmetic,
// }
//
// pub struct ComparisonOperationView {
//     left: NodeIndex,
//     right: NodeIndex,
//     operator: operator::Comparison,
// }
//
// pub struct LogicalOperationView {
//     left: NodeIndex,
//     right: NodeIndex,
//     operator: operator::Logical,
// }
//
// pub struct BinaryOperationView {
//     left: NodeIndex,
//     right: NodeIndex,
//     operator: operator::BitwiseBinary,
// }
//
// pub struct MemberAccessView {
//     parent: NodeIndex,
//     child: NodeIndex,
//     operator: operator::MemberAccess,
// }
//
// pub struct IndexOperationView {
//     parent: NodeIndex,
//     index: NodeIndex,
//     safe: bool,
// }
//
// pub struct UndefinedCoalescingView {
//     possibly_undefined: NodeIndex,
//     default: NodeIndex,
// }
//
// pub struct TypeCastingView {
//     value: NodeIndex,
//     type_: Type,
// }
//
// pub struct PrefixUnaryOperationView {
//     operand: NodeIndex,
//     operator: operator::PrefixUnary,
// }
//
// pub struct PostfixUnaryOperationView {
//     operand: NodeIndex,
//     operator: operator::PostfixUnary,
// }
//
// pub struct FunctionCallView {
//     callee: NodeIndex,
//     args: Vec<NodeIndex>,
// }
//
// pub struct MethodCallView {
//     parent: NodeIndex,
//     method_name: SourceSpan,
//     args: Vec<NodeIndex>,
//     mutable_self: bool,
// }
