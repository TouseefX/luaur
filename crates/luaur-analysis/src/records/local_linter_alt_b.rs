#[derive(Debug, Clone)]
pub struct Local {
    pub(crate) defined: bool,
    pub(crate) initialized: bool,
    pub(crate) assigned: bool,
    pub(crate) first_use: *mut luaur_ast::records::ast_expr_local::AstExprLocal,
}
