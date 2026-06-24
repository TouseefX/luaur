extern crate alloc;

use crate::records::expr_printer::ExprPrinter;
use crate::type_aliases::definition::Definition;
use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn dump_expr(
    expr: *mut AstExpr,
    use_defs: &DenseHashMap<*mut AstExpr, *mut Definition>,
) -> String {
    let mut printer = ExprPrinter::new(use_defs.clone());
    printer.visit_ast_expr(expr);
    printer.result
}
