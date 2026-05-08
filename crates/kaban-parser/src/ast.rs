use kaban_core::{ToUsize, UIndex, source::Source};
use kaban_lexer::{Token};

use crate::{node::{NodeData, NodeIndex, NodeTag, TokenIndex}};

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
