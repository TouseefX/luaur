use crate::enums::control_flow::ControlFlow;
use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl ConstraintGenerator {
    // ConstraintGenerator::visit(const ScopePtr&, AstStatReturn*)
    // (ConstraintGenerator.cpp:1961).
    pub fn visit_scope_ptr_ast_stat_return(
        &mut self,
        scope: ScopePtr,
        ret: *mut AstStatReturn,
    ) -> ControlFlow {
        // At this point, the only way scope->returnType should have anything
        // interesting in it is if the function has an explicit return annotation.
        // If this is the case, then we can expect that the return expression
        // conforms to that.
        let mut expected_types: Vec<Option<TypeId>> = Vec::new();
        let scope_ref = unsafe { &*scope.as_ref() };
        let return_type = scope_ref.return_type;
        let mut iter = begin(return_type);
        let end_iter = end(return_type);
        while iter.operator_ne(&end_iter) {
            expected_types.push(Some(*iter.operator_deref()));
            iter.operator_inc();
        }

        let list = unsafe { (*ret).list };
        let expr_types = self
            .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
                &scope,
                list,
                &expected_types,
            )
            .tp;

        self.add_constraint_scope_ptr_location_constraint_v(
            &scope,
            unsafe { (*ret).base.base.location },
            ConstraintV::PackSubtype(PackSubtypeConstraint {
                sub_pack: expr_types,
                super_pack: return_type,
                returns: true,
            }),
        );

        ControlFlow::Returns
    }
}
