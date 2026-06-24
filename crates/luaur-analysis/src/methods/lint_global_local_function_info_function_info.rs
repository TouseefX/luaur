use crate::records::function_info::FunctionInfo;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl FunctionInfo {
    pub fn function_info_ast(ast: *mut AstExprFunction) -> Self {
        Self {
            ast,
            dominated_globals: luaur_common::records::dense_hash_set::DenseHashSet::new(
                luaur_ast::records::ast_name::AstName::default(),
            ),
            conditional_execution: false,
        }
    }
}
