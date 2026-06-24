use crate::enums::value_context::ValueContext;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl TypeChecker2 {
    pub fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) {
        unsafe {
            let assign_ref = &*assign;
            let vars = assign_ref.vars;
            let values = assign_ref.values;

            let count = std::cmp::min(vars.size, values.size);

            for i in 0..count {
                let lhs = unsafe { *vars.data.add(i) };
                self.visit_ast_expr_value_context(lhs, ValueContext::LValue);
                let lhs_type = self.lookup_type(lhs);

                let rhs = unsafe { *values.data.add(i) };
                self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
                let rhs_type = self.lookup_type(rhs);

                let never_ptr = get_type_id::<NeverType>(lhs_type);
                if !never_ptr.is_null() {
                    self.report_errors_from_assigning_to_never(lhs, rhs_type);
                    continue;
                }

                if self.test_literal_or_ast_type_is_subtype(rhs, lhs_type) {
                    if let Some(binding_type) = self.get_binding_type(lhs) {
                        self.test_literal_or_ast_type_is_subtype(rhs, binding_type);
                    }
                }
            }
        }
    }
}
