use luaur_ast::records::ast_expr_global::AstExprGlobal;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Global {
    pub(crate) used: bool,
    pub(crate) builtin: bool,
    pub(crate) firstRef: *mut AstExprGlobal,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            used: false,
            builtin: false,
            firstRef: core::ptr::null_mut(),
        }
    }
}
