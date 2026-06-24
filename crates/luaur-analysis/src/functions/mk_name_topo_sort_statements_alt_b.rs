use crate::functions::mk_name_topo_sort_statements::mk_name_ast_local;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_expr_local::AstExprLocal;

pub fn mk_name_ast_expr_local(local: &AstExprLocal) -> Identifier {
    mk_name_ast_local(unsafe { &*local.local })
}
