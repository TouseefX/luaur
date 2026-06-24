use crate::records::contains_function_call::ContainsFunctionCall;

use luaur_ast::records::ast_stat_function::AstStatFunction;

impl ContainsFunctionCall {
    pub fn visit_ast_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let _node = node as *mut AstStatFunction;

        false
    }
}
