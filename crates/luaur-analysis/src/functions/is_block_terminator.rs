use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_break::AstStatBreak;
use luaur_ast::records::ast_stat_continue::AstStatContinue;
use luaur_ast::records::ast_stat_return::AstStatReturn;
use luaur_ast::rtti::ast_node_as;

pub fn is_block_terminator(stat: &AstStat) -> bool {
    let stat_ptr = stat as *const AstStat as *mut AstStat;
    unsafe {
        !ast_node_as::<AstStatReturn>(stat_ptr as *mut AstNode).is_null()
            || !ast_node_as::<AstStatBreak>(stat_ptr as *mut AstNode).is_null()
            || !ast_node_as::<AstStatContinue>(stat_ptr as *mut AstNode).is_null()
    }
}
