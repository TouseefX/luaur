use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::position::Position;

use crate::functions::is_within_comment_module::is_within_comment;

pub fn is_within_comment_parse_result_position(result: &ParseResult, pos: Position) -> bool {
    is_within_comment(&result.comment_locations, pos)
}
