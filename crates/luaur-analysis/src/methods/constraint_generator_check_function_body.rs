use crate::enums::control_flow::ControlFlow;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl ConstraintGenerator {
    pub fn check_function_body(&mut self, scope: &ScopePtr, fn_expr: &AstExprFunction) {
        let cf = self
            .visit_block_without_child_scope(scope.as_ref() as *const _ as *mut _, fn_expr.body);

        if cf == ControlFlow::None {
            let builtin_types = unsafe { &*self.builtin_types };
            let sub_pack = builtin_types.emptyTypePack;
            let super_pack = scope.return_type;

            let constraint = PackSubtypeConstraint {
                sub_pack,
                super_pack,
                returns: true,
            };

            self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                fn_expr.base.base.location,
                ConstraintV::PackSubtype(constraint),
            );
        }
    }
}
