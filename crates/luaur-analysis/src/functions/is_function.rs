use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::rtti::ast_node_is;

pub fn is_function(stat: &AstStat) -> bool {
    let node = stat as *const AstStat as *mut luaur_ast::records::ast_node::AstNode;
    unsafe { ast_node_is::<AstStatFunction>(node) || ast_node_is::<AstStatLocalFunction>(node) }
}
