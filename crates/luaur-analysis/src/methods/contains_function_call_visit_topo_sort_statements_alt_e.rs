use crate::records::contains_function_call::ContainsFunctionCall;

use core::ffi::c_void;

impl ContainsFunctionCall {
    pub fn visit_ast_expr_function(&mut self, _node: *mut c_void) -> bool {
        false
    }
}
