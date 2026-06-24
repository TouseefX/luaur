use alloc::vec::Vec;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;

use crate::records::reducer::Reducer;
use crate::type_aliases::span::Span;

impl Reducer {
    pub fn pruned_span(
        &self,
        block: *mut AstStatBlock,
        span1: Span,
        span2: Span,
    ) -> Vec<*mut AstStat> {
        let mut result: Vec<*mut AstStat> = Vec::new();

        unsafe {
            if block.is_null() {
                return result;
            }

            // AstArray fields are private; use the begin() method to get the data pointer.
            let data = (*block).body.begin();
            if data.is_null() {
                return result;
            }

            for i in span1.0..span1.1 {
                result.push(*data.add(i));
            }

            for i in span2.0..span2.1 {
                result.push(*data.add(i));
            }
        }

        result
    }
}

pub fn reducer_pruned_span(
    this: &Reducer,
    block: *mut AstStatBlock,
    span1: Span,
    span2: Span,
) -> Vec<*mut AstStat> {
    this.pruned_span(block, span1, span2)
}
