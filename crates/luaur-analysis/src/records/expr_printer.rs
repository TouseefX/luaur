use crate::type_aliases::definition::Definition;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct ExprPrinter {
    pub(crate) use_defs: DenseHashMap<*mut AstExpr, *mut Definition>,
    pub(crate) result: alloc::string::String,
}
