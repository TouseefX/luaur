use crate::records::nth::Nth;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct FindNthOccurenceOf {
    pub(crate) requested_nth: Nth,
    pub(crate) current_occurrence: i32,
    pub(crate) the_node: *mut AstNode,
}

impl AstVisitor for FindNthOccurenceOf {
    fn visit_node(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.check_it(node as *mut AstNode)
    }

    fn visit_type(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.check_it(node as *mut AstNode)
    }

    fn visit_type_pack(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.check_it(node as *mut AstNode)
    }
}

impl FindNthOccurenceOf {
    pub(crate) fn check_it(&mut self, n: *mut AstNode) -> bool {
        let node = unsafe { &*n };
        if node.class_index == self.requested_nth.class_index {
            self.current_occurrence += 1;
            if self.current_occurrence == self.requested_nth.nth {
                self.the_node = n;
                return false;
            }
        }
        true
    }
}
