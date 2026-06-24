use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

pub fn get_end_location(function: &AstExprFunction) -> Location {
    let mut loc = function.base.base.location;
    if loc.begin.line != loc.end.line {
        let mut begin = loc.end;
        begin.column = if begin.column > 3 {
            begin.column - 3
        } else {
            0
        };
        loc = Location::with_length(begin, 3);
    }

    loc
}
