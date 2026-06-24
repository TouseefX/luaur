use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;

pub unsafe fn block_statement(root: *mut AstStatBlock, index: usize) -> *mut AstStat {
    *(*root).body.data.add(index)
}
