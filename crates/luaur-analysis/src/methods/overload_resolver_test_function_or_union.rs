//! Source: `Analysis/src/OverloadResolver.cpp:550-593` (hand-ported)
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::overload_resolution::OverloadResolution;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::type_error::TypeError;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::variant::Variant2;

impl OverloadResolver {
    pub fn test_function_or_union(
        &mut self,
        result: &mut OverloadResolution,
        fn_ty: TypeId,
        args_pack: TypePackId,
        fn_location: Location,
        unique_types: *mut DenseHashSet<TypeId>,
    ) {
        let fn_ty = unsafe { follow_type_id(fn_ty) };
        LUAU_ASSERT!(fn_ty == unsafe { follow_type_id(fn_ty) });

        let ut = unsafe { get_type_id::<UnionType>(fn_ty) };
        if !ut.is_null() {
            // A union of functions is a valid overload iff every type within it is a valid overload.

            let mut inner_result = OverloadResolution {
                ok: alloc::vec::Vec::new(),
                non_functions: alloc::vec::Vec::new(),
                potential_overloads: alloc::vec::Vec::new(),
                incompatible_overloads: alloc::vec::Vec::new(),
                arity_mismatches: alloc::vec::Vec::new(),
                metamethods: DenseHashSet::new(core::ptr::null_mut()),
            };
            let options = unsafe { (*ut).options.clone() };
            let mut count: usize = 0;
            for t in options {
                count += 1;
                self.test_function_or_call_metamethod(
                    &mut inner_result,
                    t,
                    args_pack,
                    fn_location,
                    unique_types,
                );
            }

            if count == inner_result.ok.len() {
                result.ok.push(fn_ty);
            } else if count == inner_result.ok.len() + inner_result.potential_overloads.len() {
                let mut all_constraints = alloc::vec::Vec::new();
                for (_t, constraints) in inner_result.potential_overloads.iter() {
                    all_constraints.extend(constraints.iter().cloned());
                }

                result.potential_overloads.push((fn_ty, all_constraints));
            } else {
                // FIXME: We should probably report something better here, but it's
                // important for type checking that we include this.
                let errors = alloc::vec![TypeError::type_error_location_type_error_data(
                    fn_location,
                    TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
                )];
                result
                    .incompatible_overloads
                    .push((fn_ty, Variant2::V1(errors)));
            }
        } else {
            self.test_function_or_call_metamethod(
                result,
                fn_ty,
                args_pack,
                fn_location,
                unique_types,
            );
        }
    }
}
