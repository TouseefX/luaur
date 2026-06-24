#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub ast: *mut luaur_ast::records::ast_expr_function::AstExprFunction,
    pub dominated_globals:
        luaur_common::records::dense_hash_set::DenseHashSet<luaur_ast::records::ast_name::AstName>,
    pub conditional_execution: bool,
}
