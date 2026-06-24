use crate::records::type_checker_2::TypeChecker2;
use core::cmp;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

impl TypeChecker2 {
    pub fn get_end_location(&self, function: *const AstExprFunction) -> Location {
        let function = unsafe { &*function };
        let mut loc = function.base.base.location;

        if loc.begin.line != loc.end.line {
            let mut begin = loc.end;
            begin.column = cmp::max(0, begin.column as i32 - 3) as u32;
            loc = Location::new(begin, loc.end);
        }

        loc
    }
}
