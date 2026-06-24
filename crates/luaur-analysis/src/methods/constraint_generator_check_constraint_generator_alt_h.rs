use crate::enums::type_context::TypeContext;
use crate::functions::add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns;
use crate::functions::checkpoint::checkpoint;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::has_free_type::has_free_type;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::inference::Inference;
use crate::records::scope::Scope;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::rc::Rc;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_function_optional_type_id_bool(
        &mut self,
        scope: &ScopePtr,
        func: *mut AstExprFunction,
        expected_type: Option<TypeId>,
        generalize: bool,
    ) -> Inference {
        let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

        let start_checkpoint = checkpoint(self as *const _);
        let sig =
            self.check_function_signature(scope, core::ptr::null_mut(), func, expected_type, None);

        self.interior_free_types.push(Default::default());
        self.check_function_body(&sig.body_scope, unsafe { &*func });
        let end_checkpoint = checkpoint(self as *const _);

        let generalized_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };
        let gc = self.add_constraint_scope_ptr_location_constraint_v(
            &sig.signature_scope,
            unsafe { (*func).base.base.location },
            ConstraintV::Generalization(GeneralizationConstraint {
                generalized_type: generalized_ty,
                source_type: sig.signature,
                interior_types: alloc::vec::Vec::new(),
                has_deprecated_attribute: false,
                deprecated_info: Default::default(),
                no_generics: false,
            }),
        );

        unsafe {
            let signature_scope = sig.signature_scope.as_ref() as *const Scope as *mut Scope;
            (*signature_scope).interior_free_types = Some(core::mem::take(
                &mut self.interior_free_types.last_mut().unwrap().types,
            ));
            (*signature_scope).interior_free_type_packs = Some(core::mem::take(
                &mut self.interior_free_types.last_mut().unwrap().type_packs,
            ));
        }
        self.interior_free_types.pop();

        unsafe {
            let blocked = getMutable::<BlockedType>(generalized_ty);
            (*blocked).set_owner(gc as *const _);
        }

        if FFlag::LuauConstraintGraph.get() {
            add_all_as_dependencies_and_chain_returns(start_checkpoint, end_checkpoint, self, gc);
        } else {
            let mut previous: *mut Constraint = core::ptr::null_mut();
            for_each_constraint(
                start_checkpoint,
                end_checkpoint,
                self,
                |constraint: *mut Constraint| {
                    unsafe { (*gc).deprecated_dependencies.push(constraint) };

                    if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c } {
                        if psc.returns {
                            if !previous.is_null() {
                                unsafe { (*constraint).deprecated_dependencies.push(previous) };
                            }
                            previous = constraint;
                        }
                    }
                },
            );
        }

        if generalize && has_free_type(sig.signature) {
            Inference::inference_type_id_refinement_id(generalized_ty, core::ptr::null_mut())
        } else {
            Inference::inference_type_id_refinement_id(sig.signature, core::ptr::null_mut())
        }
    }
}
