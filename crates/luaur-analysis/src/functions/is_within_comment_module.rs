use luaur_ast::records::comment::Comment;
use luaur_ast::records::lexeme::Type;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

pub fn is_within_comment(comment_locations: &alloc::vec::Vec<Comment>, pos: Position) -> bool {
    // Build a sentinel Comment to use with lower_bound
    let sentinel = Comment {
        r#type: Type::Comment,
        location: Location::new(pos, pos),
    };

    // Find the first comment whose end is >= pos
    let iter = comment_locations.iter().position(|c| {
        if c.r#type == Type::Comment {
            c.location.end.line >= pos.line
        } else {
            c.location.end >= pos
        }
    });

    if let Some(idx) = iter {
        if contains(pos, comment_locations[idx]) {
            return true;
        }

        // Try the next comment, if it exists
        let idx = idx + 1;
        if idx < comment_locations.len() && contains(pos, comment_locations[idx]) {
            return true;
        }
    }

    false
}

fn contains(pos: Position, comment: Comment) -> bool {
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

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use is_within_comment as is_within_comment_vector_comment_position;
