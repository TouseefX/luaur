use crate::functions::find_node_at_position_ast_query::find_node_at_position_source_module_position;
use crate::records::source_module::SourceModule;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::position::Position;

pub fn find_expr_at_position(source: &SourceModule, pos: Position) -> *mut AstExpr {
    let node = find_node_at_position_source_module_position(source, pos);
    if !node.is_null() {
        unsafe { (*node).as_expr() }
    } else {
        core::ptr::null_mut()
    }
}
