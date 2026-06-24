use crate::records::expr_printer::ExprPrinter;
use crate::type_aliases::definition::Definition;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ExprPrinter {
    pub fn new(use_defs: DenseHashMap<*mut AstExpr, *mut Definition>) -> Self {
        Self {
            use_defs,
            result: alloc::string::String::new(),
        }
    }
}
