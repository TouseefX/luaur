use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

pub fn stat_is_before_pos(stat: &AstNode, cursor_pos: &Position) -> bool {
    let stat_location = unsafe { &*stat }.location;
    stat_location.begin < *cursor_pos
}
