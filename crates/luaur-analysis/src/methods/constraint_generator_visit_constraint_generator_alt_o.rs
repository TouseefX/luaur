// ConstraintGenerator::visit(const ScopePtr&, AstStatTypeFunction*) (ConstraintGenerator.cpp:2194-2271).
use crate::enums::control_flow::ControlFlow;
use crate::functions::add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns;
use crate::functions::checkpoint::checkpoint;
use crate::functions::follow_type::follow;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_type_alt_j::get as get_type;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::reserved_identifier::ReservedIdentifier;
use crate::records::scope::Scope;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::rc::Rc;
use alloc::string::String;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_type_function(
        &mut self,
        scope: &ScopePtr,
        function: *mut AstStatTypeFunction,
    ) -> ControlFlow {
        let function_ref = unsafe { &*function };

        // function->name == "typeof"
        let name_bytes = unsafe { core::ffi::CStr::from_ptr(function_ref.name.value) }.to_bytes();
        if name_bytes == b"typeof" {
            self.report_error(
                function_ref.base.base.location,
                TypeErrorData::ReservedIdentifier(ReservedIdentifier::new(String::from("typeof"))),
            );
        }

        let scope_it = self
            .ast_type_function_environment_scopes
            .find(&(function as *const AstStatTypeFunction))
            .cloned();
        LUAU_ASSERT!(scope_it.is_some());

        let environment_scope: ScopePtr = scope_it.unwrap().unwrap();

        let start_checkpoint = checkpoint(self as *const _);
        let sig = self.check_function_signature(
            &environment_scope,
            core::ptr::null_mut(),
            function_ref.body,
            None,
            None,
        );

        // Place this function as a child of the non-type function scope.
        unsafe {
            (*(scope.as_ref() as *const Scope as *mut Scope))
                .children
                .push(sig.signature_scope.as_ref() as *const Scope as *mut Scope);
        }
        self.interior_free_types.push(Default::default());
        self.check_function_body(&sig.body_scope, unsafe { &*function_ref.body });
        let end_checkpoint = checkpoint(self as *const _);

        let generalized_ty: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
        let gc: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
            &sig.signature_scope,
            function_ref.base.base.location,
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

        unsafe {
            let blocked = getMutable::<BlockedType>(generalized_ty);
            (*blocked).set_owner(gc as *const _);
        }
        self.interior_free_types.pop();

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

        let existing_function_ty = unsafe {
            (*(environment_scope.as_ref() as *const Scope as *mut Scope)).lookup_symbol(
                crate::records::symbol::Symbol::from_global(function_ref.name),
            )
        };

        if existing_function_ty.is_none() {
            unsafe {
                (*self.ice).ice_string_location(
                    "checkAliases did not populate type function name",
                    &function_ref.name_location,
                );
            }
        }

        let unpacked_ty: TypeId = unsafe { follow(existing_function_ty.unwrap()) };

        let bt = unsafe { get_type::<BlockedType>(unpacked_ty) };
        if !bt.is_null() && unsafe { (*bt).getOwner() }.is_null() {
            unsafe {
                (*crate::functions::as_mutable_type::as_mutable_type_id(unpacked_ty)).ty =
                    TypeVariant::Bound(generalized_ty);
            }
        }

        ControlFlow::None
    }
}
