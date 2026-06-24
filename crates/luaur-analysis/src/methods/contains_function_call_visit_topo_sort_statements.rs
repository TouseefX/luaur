use crate::records::contains_function_call::ContainsFunctionCall;

impl ContainsFunctionCall {
    pub fn visit_ast_expr(&mut self, _node: *mut core::ffi::c_void) -> bool {
        !self.result
    }
}
