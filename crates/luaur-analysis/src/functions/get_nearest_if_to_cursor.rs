use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

pub fn get_nearest_if_to_cursor(stmt: *mut AstStat, cursor_pos: &Position) -> *mut AstStatIf {
    unsafe {
        let mut current: *mut AstStatIf = ast_node_as::<AstStatIf>(stmt as *mut AstNode);

        if current.is_null() {
            return core::ptr::null_mut();
        }

        while ast_node_is::<AstStatIf>(&(*current).base.base) {
            let elsebody = (*current).elsebody;
            if !elsebody.is_null() && (*elsebody).base.location.containsClosed(*cursor_pos) {
                let else_if_s = ast_node_as::<AstStatIf>(elsebody as *mut AstNode);
                if !else_if_s.is_null() {
                    current = else_if_s;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        current
    }
}
