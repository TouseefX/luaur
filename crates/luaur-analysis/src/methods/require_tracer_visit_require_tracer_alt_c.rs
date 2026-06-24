use crate::records::require_tracer::RequireTracer;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl RequireTracer {
    pub fn visit_ast_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        let stat = node as *mut AstStatLocal;
        let stat_ref = unsafe { &*stat };

        let vars_size = stat_ref.vars.size;
        let values_size = stat_ref.values.size;
        let limit = if vars_size < values_size {
            vars_size
        } else {
            values_size
        };

        for i in 0..limit {
            let local = unsafe { *stat_ref.vars.data.add(i) };
            let expr = unsafe { *stat_ref.values.data.add(i) };

            self.locals.try_insert(local, expr);
        }

        true
    }
}
