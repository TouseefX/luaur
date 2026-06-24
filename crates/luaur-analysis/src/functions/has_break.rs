use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_break::AstStatBreak;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

pub fn has_break(node: *mut AstStat) -> bool {
    if node.is_null() {
        return false;
    }

    unsafe {
        if !ast_node_as::<AstStatBlock>(node as *mut luaur_ast::records::ast_node::AstNode).is_null()
        {
            let stat =
                ast_node_as::<AstStatBlock>(node as *mut luaur_ast::records::ast_node::AstNode);
            if stat.is_null() {
                return false;
            }

            for i in 0..(*stat).body.size {
                if has_break(*(*stat).body.data.add(i)) {
                    return true;
                }
            }
            return false;
        }

        if ast_node_is::<AstStatBreak>(unsafe {
            &*(node as *const luaur_ast::records::ast_node::AstNode)
        }) {
            return true;
        }

        if !ast_node_as::<AstStatIf>(node as *mut luaur_ast::records::ast_node::AstNode).is_null() {
            let stat = ast_node_as::<AstStatIf>(node as *mut luaur_ast::records::ast_node::AstNode);
            if stat.is_null() {
                return false;
            }

            if has_break((*stat).thenbody as *mut AstStat) {
                return true;
            }

            if !(*stat).elsebody.is_null() && has_break((*stat).elsebody as *mut AstStat) {
                return true;
            }

            return false;
        }

        false
    }
}
