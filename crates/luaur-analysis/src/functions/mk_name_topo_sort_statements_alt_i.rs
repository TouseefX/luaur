use crate::functions::mk_name_topo_sort_statements::mk_name_ast_local;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

pub fn mk_name_ast_stat_local_function(function: &AstStatLocalFunction) -> Identifier {
    unsafe { mk_name_ast_local(&*function.name) }
}
