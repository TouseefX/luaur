use crate::records::contains_function_call::ContainsFunctionCall;

impl ContainsFunctionCall {
    pub fn visit_ast_stat_for_in(&mut self, _node: *mut core::ffi::c_void) -> bool {
        self.result = true;
        false
    }
}
