use kaban_core::{ToBool, ToUsize, UIndex, source::Source};
use kaban_lexer::{Token};

use crate::node::{NodeData, NodeIndex, NodeIndexVec, NodeTag, ToOption, ToWrapper, TokenIndex};

pub struct AST<'a> {
    tokens: &'a [Token],
    node_tags: Vec<NodeTag>,
    node_data: Vec<NodeData>,
    extra: Vec<UIndex>,
    source: Source<'a>,
    pub root: NodeIndex,
}

impl<'a> AST<'a> {
    pub fn new(
        tokens: &'a [Token], 
        node_tags: Vec<NodeTag>, 
        node_data: Vec<NodeData>,
        extra: Vec<UIndex>,
        source: Source<'a>,
        root: NodeIndex,
    ) -> Self {
        Self { tokens, node_tags, node_data, extra, source, root }
    }

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
    pub fn get_left_right(&self, index: NodeIndex) -> (UIndex, UIndex) {
        let data = self.get_data(index);
        (data.left, data.right)
    }

    #[inline(always)]
    pub fn get_one_extra(&self, index: UIndex) -> UIndex {
        self.extra[index.usize()]
    }

    #[inline(always)]
    pub fn get_extra_span(&self, start: UIndex, end: UIndex) -> &[UIndex] {
        &self.extra[start.usize()..end.usize()]
    }

    #[inline(always)]
    pub fn get_extra_from_count(&self, count: UIndex, start: UIndex) -> &[UIndex] {
        &self.extra[start.usize()..(start + count).usize()]
    }

    pub fn get_source(&self) -> Source<'a> {
        self.source
    }
}


///VIEWS
impl<'a> AST<'a> {
    pub fn view_func_call(&'a self, index: NodeIndex) -> FuncCall<'a> {
        debug_assert!(NodeTag::FuncCall == self.get_tag(index));
        let (callee, extra) = self.get_left_right(index);
        let arg_count = self.get_one_extra(extra);
        let args_start = extra + 1;
        FuncCall {
            callee: callee.node_index(),
            args: self.get_extra_from_count(arg_count, args_start).node_index_slice(),
        }
    }

    pub fn view_method_call(&'a self, index: NodeIndex) -> MethodCall<'a> {
        debug_assert!(NodeTag::MethodCall == self.get_tag(index));
        let (callee, extra) = self.get_left_right(index);
        let method_name = extra;
        let is_self_mut = extra + 1;
        let arg_count = extra + 2;
        let args = extra + 3;

        let arg_count = self.get_one_extra(arg_count);
        MethodCall {
            callee: callee.node_index(),
            method_name: self.get_one_extra(method_name).node_index(),
            args: self.get_extra_from_count(arg_count, args).node_index_slice(),
            is_self_mut: self.get_one_extra(is_self_mut).bool(),
        }
    }

    pub fn view_index(&'a self, index: NodeIndex) -> Index {
        debug_assert!(NodeTag::Index == self.get_tag(index));
        let (callee, extra) = self.get_left_right(index);
        let is_safe_index = extra;
        let index_by= extra + 1;
        Index {
            callee: callee.node_index(),
            is_safe_index: self.get_one_extra(is_safe_index).bool(),
            index_by: self.get_one_extra(index_by).node_index(),
        }
    }

        
    pub fn view_array_lit(&'a self, index: NodeIndex) -> ArrayLit<'a> {
        debug_assert!(NodeTag::ArrayLit == self.get_tag(index));
        let general_list = self.view_general_list(index);
        ArrayLit {
            len: general_list.len,
            expressions: general_list.indices.node_index_slice(),
        }
    }

    pub fn view_union(&'a self, index: NodeIndex) -> Union<'a> {
        debug_assert!(NodeTag::Union == self.get_tag(index));
        let general_list = self.view_general_list(index);
        Union {
            len: general_list.len,
            types: general_list.indices.node_index_slice(),
        }
    }

    pub fn view_block(&'a self, index: NodeIndex) -> Block<'a> {
        debug_assert!(NodeTag::Block == self.get_tag(index));
        let general_list = self.view_general_list(index);
        Block {
            len: general_list.len,
            statements: general_list.indices.node_index_slice(),
        }
    }

    ///General lists are defined as the having an arg count on the left side and a index to extra
    ///on the right side.
    ///
    ///eg. [NodeTag::ArrayLit]
    pub fn view_general_list(&'a self, index: NodeIndex) -> GeneralList<'a> {
        // debug_assert!(NodeTag::MethodCall == self.get_tag(index));
        let (list_len, extra) = self.get_left_right(index);
        GeneralList {
            len: list_len,
            indices: self.get_extra_from_count(list_len, extra),
        }
    }

    pub fn view_for_expression(&'a self, index: NodeIndex) -> For {
        debug_assert!(NodeTag::For == self.get_tag(index));
        let (binding, extra_pointer) = self.get_left_right(index);
        let iterator = extra_pointer;
        let block = extra_pointer + 1;
        For {
            binding: binding.node_index(),
            iterator: self.get_one_extra(iterator).node_index(),
            block: self.get_one_extra(block).node_index(),
        }
    }

    pub fn view_if_expression(&'a self, index: NodeIndex) -> If {
        debug_assert!(NodeTag::If == self.get_tag(index));
        let (condition, extra_pointer) = self.get_left_right(index);
        let then_pointer = extra_pointer;
        let else_pointer = extra_pointer + 1;
        let else_ = self.get_one_extra(else_pointer).node_index(); 
        If {
            condition: condition.node_index(),
            then: self.get_one_extra(then_pointer).node_index(),
            else_: else_.to_option(),
        }
    }
    
    pub fn view_let_statement(&'a self, index: NodeIndex) -> Let {
        debug_assert!(NodeTag::Let == self.get_tag(index));
        let (name, extra_pointer) = self.get_left_right(index);
        let type_ = extra_pointer;
        let assignment = extra_pointer + 1; 
        Let {
            name: name.node_index(),
            type_: self.get_one_extra(type_).node_index().to_option(),
            assignment: self.get_one_extra(assignment).node_index(),
        }
    }

    pub fn view_match_expression(&'a self, index: NodeIndex) -> Match<'a> {
        debug_assert!(NodeTag::Match == self.get_tag(index));
        let (target, extra_pointer) = self.get_left_right(index);
        let arm_count = self.get_one_extra(extra_pointer);
        let arm_list_start = extra_pointer + 1; 
        let arms = self.get_extra_from_count(arm_count, arm_list_start);
        Match {
            target: target.node_index(),
            arms: arms.node_index_slice(),
        }
    }

    pub fn view_struct_instantiation(&'a self, index: NodeIndex) -> StructInstantiation<'a> {
        debug_assert!(NodeTag::StructInstantiation == self.get_tag(index));
        let (struct_name, extra_pointer) = self.get_left_right(index);
        let field_count = self.get_one_extra(extra_pointer);
        let field_start = extra_pointer + 1;
        let field_instantiation = self.get_extra_from_count(field_count, field_start);
        StructInstantiation {
            struct_name: struct_name.node_index().to_option(),
            field_instantiation: field_instantiation.node_index_slice(),
        }
    }
}

//These structs are temporary data holders meant to construct nodes on demand for quick viewing.
//Only complex nodes get structs, the nodes with data in extra.
//Simple nodes (binary ops, leaves) are read directly via `get_left_right` without a view struct.

pub struct FuncCall<'a> {
    pub callee: NodeIndex,
    pub args: &'a [NodeIndex]
}

pub struct MethodCall<'a> {
    pub callee: NodeIndex,
    pub method_name: NodeIndex,
    pub args: &'a [NodeIndex],
    pub is_self_mut: bool,
}

pub struct Index {
    pub callee: NodeIndex,
    pub index_by: NodeIndex,
    pub is_safe_index: bool,
}

pub struct ArrayLit<'a> {
    pub len: UIndex,
    pub expressions: &'a [NodeIndex],
}

pub struct Union<'a> {
    pub len: UIndex,
    pub types: &'a [NodeIndex],
}

pub struct Block<'a> {
    pub len: UIndex,
    pub statements: &'a [NodeIndex],
}

pub struct GeneralList<'a> {
    pub len: UIndex,
    pub indices: &'a [UIndex],
}

pub struct For {
    pub binding: NodeIndex,
    pub iterator: NodeIndex,
    pub block: NodeIndex,
}

pub struct If {
    pub condition: NodeIndex,
    pub then: NodeIndex,
    pub else_: Option<NodeIndex>,
}

pub struct Let {
    pub name: NodeIndex,
    pub type_: Option<NodeIndex>,
    pub assignment: NodeIndex,
}

pub struct Match<'a> {
    pub target: NodeIndex,
    pub arms: &'a[NodeIndex],
}

pub struct StructInstantiation<'a> {
    pub struct_name: Option<NodeIndex>,
    pub field_instantiation: &'a[NodeIndex],
}
