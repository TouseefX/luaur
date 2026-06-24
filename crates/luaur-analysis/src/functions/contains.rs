use luaur_ast::records::comment::Comment;
use luaur_ast::records::lexeme::Type;
use luaur_ast::records::position::Position;

pub fn contains(pos: Position, comment: Comment) -> bool {
    if comment.location.contains(pos) {
        return true;
    } else if comment.r#type == Type::BrokenComment && comment.location.begin <= pos {
        return true;
    } else if comment.r#type == Type::Comment
        && comment.location.end.line == pos.line
        && comment.location.begin <= pos
    {
        return true;
    } else {
        return false;
    }
}
