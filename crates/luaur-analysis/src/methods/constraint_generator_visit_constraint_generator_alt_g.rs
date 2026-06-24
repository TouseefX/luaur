// ConstraintGenerator::visit(const ScopePtr&, AstStatLocalFunction*) (ConstraintGenerator.cpp:1733-1797).
use crate::enums::control_flow::ControlFlow;
use crate::functions::add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns;
use crate::functions::checkpoint::checkpoint;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::propagate_deprecated_attribute_to_constraint::propagate_deprecated_attribute_to_constraint;
use crate::records::binding::Binding;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::rc::Rc;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_local_function(
        &mut self,
        scope: &ScopePtr,
        function: *mut AstStatLocalFunction,
    ) -> ControlFlow {
        let function_ref = unsafe { &*function };
        let name_local = function_ref.name;
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

        // The parser ensures that every local function has a distinct Symbol for its name.
        let ty = unsafe {
            (*scope_raw).lookup_symbol(crate::records::symbol::Symbol::from_local(name_local))
        };
        LUAU_ASSERT!(ty.is_none());

        let function_type: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
        unsafe {
            (*scope_raw).bindings.insert(
                crate::records::symbol::Symbol::from_local(name_local),
                Binding {
                    type_id: function_type,
                    location: (*name_local).location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                },
            );
        }

        let sig = self.check_function_signature(
            scope,
            core::ptr::null_mut(),
            function_ref.func,
            None,
            Some(unsafe { (*name_local).location }),
        );
        let body_scope_raw = sig.body_scope.as_ref() as *const Scope as *mut Scope;
        unsafe {
            (*body_scope_raw).bindings.insert(
                crate::records::symbol::Symbol::from_local(name_local),
                Binding {
                    type_id: sig.signature,
                    location: (*name_local).location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                },
            );
        }

        let def = unsafe { (*self.dfg).get_def_local(name_local) };
        unsafe {
            *(*scope_raw).lvalue_types.get_or_insert(def) = function_type;
        }
        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, function_type);
        unsafe {
            *(*body_scope_raw).lvalue_types.get_or_insert(def) = sig.signature;
        }
        self.update_r_value_refinements_scope_ptr_def_id_type_id(
            &sig.body_scope,
            def,
            sig.signature,
        );

        let start = checkpoint(self as *const _);
        self.check_function_body(&sig.body_scope, unsafe { &*function_ref.func });
        let end = checkpoint(self as *const _);

        // constraintScope = sig.signatureScope ? sig.signatureScope : sig.bodyScope.
        let constraint_scope: &ScopePtr = &sig.signature_scope;

        let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
            constraint_scope,
            unsafe { (*name_local).location },
            ConstraintV::Generalization(GeneralizationConstraint {
                generalized_type: function_type,
                source_type: sig.signature,
                interior_types: alloc::vec::Vec::new(),
                has_deprecated_attribute: false,
                deprecated_info: Default::default(),
                no_generics: false,
            }),
        );

        propagate_deprecated_attribute_to_constraint(unsafe { &mut (*c).c }, function_ref.func);

        if FFlag::LuauConstraintGraph.get() {
            add_all_as_dependencies_and_chain_returns(start, end, self, c);
        } else {
            let mut previous: *mut Constraint = core::ptr::null_mut();
            for_each_constraint(start, end, self, |constraint: *mut Constraint| {
                unsafe { (*c).deprecated_dependencies.push(constraint) };
                if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c } {
                    if psc.returns {
                        if !previous.is_null() {
                            unsafe { (*constraint).deprecated_dependencies.push(previous) };
                        }
                        previous = constraint;
                    }
                }
            });
        }

        unsafe {
            let blocked = getMutable::<BlockedType>(function_type);
            (*blocked).set_owner(c as *const _);
        }

        // module->astTypes[function->func] = functionType;
        let module = self.module.as_ref().unwrap();
        let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
        unsafe {
            *(*module_ptr).ast_types.get_or_insert(
                function_ref.func as *const _ as *const luaur_ast::records::ast_expr::AstExpr,
            ) = function_type;
        }

        ControlFlow::None
    }
}
