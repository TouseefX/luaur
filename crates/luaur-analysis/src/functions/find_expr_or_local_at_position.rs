use crate::records::expr_or_local::ExprOrLocal;
use crate::records::find_expr_or_local::FindExprOrLocal;
use crate::records::source_module::SourceModule;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_ast::records::position::Position;

pub fn find_expr_or_local_at_position(source: &SourceModule, pos: Position) -> ExprOrLocal {
    let mut find_visitor = FindExprOrLocal::new(pos);
    unsafe { find_visitor.visit_stat_block(source.root as *mut core::ffi::c_void) };
    find_visitor.result
}
