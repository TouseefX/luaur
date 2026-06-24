use alloc::vec::Vec;

use crate::functions::nth::AstNodeClass;
use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
use crate::records::nth::Nth;
use luaur_ast::records::ast_node::AstNode;

pub fn query<T: AstNodeClass>(mut node: *mut AstNode, nths: Vec<Nth>) -> *mut T {
    for nth in nths {
        if node.is_null() {
            return core::ptr::null_mut();
        }

        let mut finder = FindNthOccurenceOf::new(nth);
        finder.visit_ast_node(node);

        node = finder.the_node;
    }

    if node.is_null() {
        core::ptr::null_mut()
    } else {
        node as *mut T
    }
}
