use crate::functions::is_toposortable_node::is_toposortable_node;
use luaur_ast::records::ast_stat::AstStat;

pub fn contains_toposortable_node(block: &alloc::vec::Vec<AstStat>) -> bool {
    for stat in block {
        if is_toposortable_node(stat) {
            return true;
        }
    }

    false
}
