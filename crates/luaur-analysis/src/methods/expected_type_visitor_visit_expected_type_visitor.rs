use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl ExpectedTypeVisitor {
    pub fn visit_ast_stat_assign(&mut self, stat: *mut AstStatAssign) -> bool {
        unsafe {
            let stat_ref = &*stat;
            let max_idx = core::cmp::min(stat_ref.vars.size, stat_ref.values.size);

            for idx in 0..max_idx {
                let var = *stat_ref.vars.data.add(idx);
                let value = *stat_ref.values.data.add(idx);

                if let Some(&lhs_type) = (*self.ast_types).find(&(var as *const _)) {
                    self.apply_expected_type(lhs_type, value as *const _);
                }
            }
        }

        true
    }
}
