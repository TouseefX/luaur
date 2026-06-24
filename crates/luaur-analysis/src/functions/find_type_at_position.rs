//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AstQuery.cpp:307:find_type_at_position`
//! Source: `Analysis/src/AstQuery.cpp:307-316` (hand-ported)

use crate::functions::find_expr_at_position::find_expr_at_position;
use crate::records::module::Module;
use crate::records::source_module::SourceModule;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::position::Position;

pub fn find_type_at_position(
    module: &Module,
    source_module: &SourceModule,
    pos: Position,
) -> Option<TypeId> {
    let expr = find_expr_at_position(source_module, pos);
    if !expr.is_null() {
        if let Some(&ty) = module
            .ast_types
            .find(&(expr as *const luaur_ast::records::ast_expr::AstExpr))
        {
            return Some(ty);
        }
    }
    None
}
