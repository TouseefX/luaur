use core::cmp;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_expr::AstStatExpr;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_is;

pub fn lint_implicit_return_get_end_location(
    this: &mut crate::records::lint_implicit_return::LintImplicitReturn,
    node: *const core::ffi::c_void,
) -> Location {
    let node = node as *const AstStat;
    let node_ref = unsafe { &*node };
    let loc = node_ref.base.location;

    if ast_node_is::<AstStatExpr>(unsafe { &node_ref.base })
        || ast_node_is::<AstStatAssign>(unsafe { &node_ref.base })
        || ast_node_is::<AstStatLocal>(unsafe { &node_ref.base })
    {
        return loc;
    }

    if loc.begin.line == loc.end.line {
        return loc;
    }

    let column = cmp::max(0, loc.end.column as i32 - 3) as u32;
    Location::new(Position::new(loc.end.line, column), loc.end)
}
