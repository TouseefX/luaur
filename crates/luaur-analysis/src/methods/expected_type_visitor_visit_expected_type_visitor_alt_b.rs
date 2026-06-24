use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl ExpectedTypeVisitor {
    pub fn visit_ast_stat_local(&mut self, stat: *mut AstStatLocal) -> bool {
        unsafe {
            let stat_ref = &*stat;
            let max_idx = core::cmp::min(stat_ref.vars.size, stat_ref.values.size);

            for idx in 0..max_idx {
                let var = *stat_ref.vars.data.add(idx);
                let value = *stat_ref.values.data.add(idx);

                if let Some(&annot) =
                    (*self.ast_resolved_types).find(&((*var).annotation as *const _))
                {
                    self.apply_expected_type(annot, value as *const _);
                }
            }
        }

        true
    }
}
