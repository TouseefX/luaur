use crate::records::contains_function_call::ContainsFunctionCall;

impl ContainsFunctionCall {
    pub fn visit_ast_stat_return(&mut self, node: *mut core::ffi::c_void) -> bool {
        if self.also_return {
            self.result = true;
            return false;
        }

        self.visit_ast_stat_return(node)
    }
}
