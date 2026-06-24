use crate::functions::mk_name_topo_sort_statements::mk_name_ast_local;
use crate::functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

pub fn mk_name_ast_stat_assign(assign: &AstStatAssign) -> Option<Identifier> {
    if assign.vars.size != 1 {
        return None;
    }

    let var_ptr = unsafe { *assign.vars.data };
    if var_ptr.is_null() {
        return None;
    }

    mk_name_ast_expr(unsafe { &*var_ptr })
        .map(|id| Identifier::new(id.name().to_string(), id.ctx()))
}
