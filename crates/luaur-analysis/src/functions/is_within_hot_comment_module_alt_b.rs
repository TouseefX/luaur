use crate::functions::is_within_hot_comment_module::is_within_hot_comment_vector_hot_comment_position;
use crate::records::source_module::SourceModule;
use luaur_ast::records::position::Position;

pub fn is_within_hot_comment_source_module_position(
    source_module: &SourceModule,
    pos: Position,
) -> bool {
    is_within_hot_comment_vector_hot_comment_position(&source_module.hotcomments, pos)
}
