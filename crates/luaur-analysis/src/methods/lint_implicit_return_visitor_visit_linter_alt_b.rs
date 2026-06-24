use crate::records::visitor::Visitor;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl Visitor {
    pub fn visit_ast_stat_return(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstStatReturn;

        unsafe {
            if self.result.is_null() && (*node).list.len() > 0 {
                self.result = node;
            }
        }

        false
    }
}
