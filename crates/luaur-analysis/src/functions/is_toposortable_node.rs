use crate::functions::is_function::is_function;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_ast::rtti::ast_node_is;

pub fn is_toposortable_node(stat: &AstStat) -> bool {
    is_function(stat)
        || ast_node_is::<AstStatTypeAlias>(unsafe {
            &*(stat as *const AstStat as *mut AstStat as *mut luaur_ast::records::ast_node::AstNode)
        })
}
