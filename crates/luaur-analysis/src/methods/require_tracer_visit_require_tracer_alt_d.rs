use crate::records::require_tracer::RequireTracer;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::rtti::ast_node_as;

impl RequireTracer {
    pub fn visit_ast_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        let stat = node as *mut AstStatAssign;
        let stat_ref = unsafe { &*stat };

        for i in 0..stat_ref.vars.size {
            let var = unsafe { *stat_ref.vars.data.add(i) };
            let expr_local = unsafe {
                ast_node_as::<AstExprLocal>(var as *mut luaur_ast::records::ast_node::AstNode)
            };

            if !expr_local.is_null() {
                let local = unsafe { (*expr_local).local };
                self.locals.try_insert(local, core::ptr::null_mut());
            }
        }

        true
    }
}
