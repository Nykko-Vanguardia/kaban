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
