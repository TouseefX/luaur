use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
use crate::records::nth::Nth;
use luaur_ast::records::ast_node::AstNode;

impl FindNthOccurenceOf {
    pub fn new(nth: Nth) -> Self {
        Self {
            requested_nth: nth,
            current_occurrence: 0,
            the_node: core::ptr::null_mut::<AstNode>(),
        }
    }
}
