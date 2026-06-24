use crate::records::usage_finder::UsageFinder;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;

impl UsageFinder {
    pub fn visit_ast_expr_constant_string(&mut self, expr: *mut AstExprConstantString) -> bool {
        let expr_ref = unsafe { &*expr };
        let value_slice = unsafe {
            core::slice::from_raw_parts(
                expr_ref.value.data as *const u8,
                expr_ref.value.size as usize,
            )
        };
        let name = core::str::from_utf8(value_slice).unwrap_or("");
        self.referenced_bindings.push(name.into());
        true
    }
}
