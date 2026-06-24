//! Source: `Analysis/src/OverloadResolver.cpp:457-548` (hand-ported)
//!
//! Test a single FunctionType against an argument list. Reduces type functions
//! and does a proper arity check.
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::are_unsatisfied_arguments_optional::are_unsatisfied_arguments_optional;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::ignore_reasoning_for_return_type::ignore_reasoning_for_return_type;
use crate::functions::reduce_type_functions_type_function::reduce_type_functions;
use crate::records::blocked_type::BlockedType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::overload_resolution::OverloadResolution;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_error::TypeError;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::variant::Variant2;

impl OverloadResolver {
    pub fn test_function(
        &mut self,
        result: &mut OverloadResolution,
        fn_ty: TypeId,
        args_pack: TypePackId,
        fn_location: Location,
        unique_types: *mut DenseHashSet<TypeId>,
    ) {
        let fn_ty = unsafe { follow_type_id(fn_ty) };

        // TODO: This seems like the wrong spot to do this check.
        if unsafe {
            !get_type_id::<FreeType>(fn_ty).is_null()
                || !get_type_id::<BlockedType>(fn_ty).is_null()
                || !get_type_id::<PendingExpansionType>(fn_ty).is_null()
        } {
            // TODO.  Luckily, these constraints are not yet used.
            let constraints = alloc::vec::Vec::new();
            result.potential_overloads.push((fn_ty, constraints));
            return;
        }

        let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(fn_ty) };
        if !tfit.is_null() && unsafe { (*tfit).state } == TypeFunctionInstanceState::Unsolved {
            // TODO.  Luckily, these constraints are not yet used.
            let constraints = alloc::vec::Vec::new();
            result.potential_overloads.push((fn_ty, constraints));
            return;
        }

        let ftv = unsafe { get_type_id::<FunctionType>(fn_ty) };
        if ftv.is_null() {
            result.non_functions.push(fn_ty);
            return;
        }
        let ftv = unsafe { &*ftv };

        if !self.is_arity_compatible(args_pack, ftv.arg_types, self.builtin_types) {
            result.arity_mismatches.push(fn_ty);
            return;
        }

        let context = TypeFunctionContext {
            arena: unsafe { NonNull::new_unchecked(self.arena) },
            builtins: unsafe { NonNull::new_unchecked(self.builtin_types) },
            scope: unsafe { NonNull::new_unchecked(self.scope) },
            normalizer: unsafe { NonNull::new_unchecked(self.normalizer) },
            type_function_runtime: unsafe { NonNull::new_unchecked(self.type_function_runtime) },
            ice: unsafe { NonNull::new_unchecked(self.ice) },
            limits: unsafe { NonNull::new_unchecked(&self.limits as *const _ as *mut _) },
            subtyping: unsafe { NonNull::new_unchecked(&mut self.subtyping as *mut _) },
            solver: core::ptr::null_mut(),
            constraint: core::ptr::null(),
            user_func_name: None,
            fresh_instances: alloc::vec::Vec::new(),
        };
        let mut context = context;
        let reduce_result = reduce_type_functions(
            fn_ty,
            self.call_loc,
            unsafe { NonNull::new_unchecked(&mut context as *mut _) },
            /*force=*/ true,
        );
        if !reduce_result.errors.is_empty() {
            result
                .incompatible_overloads
                .push((fn_ty, Variant2::V1(reduce_result.errors)));
            return;
        }

        let prospective_function = unsafe {
            (*self.arena).add_type(FunctionType::function_type_new(
                args_pack,
                (*self.builtin_types).anyTypePack,
                None,
                false,
            ))
        };

        self.subtyping.unique_types = unique_types as *const DenseHashSet<TypeId>;
        let scope = self.scope;
        let mut r = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
            fn_ty,
            prospective_function,
            scope,
        );

        // Frustratingly, subtyping does not know about error suppression, so this
        // subtype test will probably fail due to the mismatched return types. Here,
        // we'll prune any SubtypingReasons that have anything to do with the return
        // type.
        ignore_reasoning_for_return_type(&mut r);

        if r.is_subtype {
            if r.assumed_constraints.is_empty() {
                result.ok.push(fn_ty);
            } else {
                result
                    .potential_overloads
                    .push((fn_ty, core::mem::take(&mut r.assumed_constraints)));
            }
        } else if !r.generic_bounds_mismatches.is_empty() {
            let mut errors: ErrorVec = alloc::vec::Vec::new();
            for gbm in r.generic_bounds_mismatches.iter() {
                errors.push(TypeError::type_error_location_type_error_data(
                    fn_location,
                    crate::type_aliases::type_error_data::TypeErrorData::GenericBoundsMismatch(
                        gbm.clone(),
                    ),
                ));
            }
            result
                .incompatible_overloads
                .push((fn_ty, Variant2::V1(errors)));
        } else if are_unsatisfied_arguments_optional(&r.reasoning, args_pack, ftv.arg_types) {
            // Important!  Subtyping doesn't know anything about
            // optional arguments.  If the only reason subtyping
            // failed is because optional arguments were not provided,
            // then this overload is actually okay.
            if r.assumed_constraints.is_empty() {
                result.ok.push(fn_ty);
            } else {
                result
                    .potential_overloads
                    .push((fn_ty, core::mem::take(&mut r.assumed_constraints)));
            }
        } else {
            result
                .incompatible_overloads
                .push((fn_ty, Variant2::V0(core::mem::take(&mut r.reasoning))));
        }
    }
}
