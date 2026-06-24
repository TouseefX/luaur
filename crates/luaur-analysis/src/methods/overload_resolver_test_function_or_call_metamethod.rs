//! Source: `Analysis/src/OverloadResolver.cpp:604-643` (hand-ported)
//!
//! A utility function for ::resolveOverload. If a particular overload is a table
//! with a __call metamethod, unwrap that and test it.
//!
//! Note: The __call metamethod can itself be overloaded, but it cannot be a
//! table that overloads __call.  It must be an actual function.
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::overload_resolution::OverloadResolution;
use crate::records::overload_resolver::OverloadResolver;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl OverloadResolver {
    pub fn test_function_or_call_metamethod(
        &mut self,
        result: &mut OverloadResolution,
        fn_ty: TypeId,
        args_pack: TypePackId,
        fn_location: Location,
        unique_types: *mut DenseHashSet<TypeId>,
    ) {
        let mut fn_ty = unsafe { follow_type_id(fn_ty) };
        let mut args_pack = args_pack;

        let mut dummy_errors: ErrorVec = alloc::vec::Vec::new();
        if let Some(call_metamethod) = find_metatable_entry(
            self.builtin_types,
            &mut dummy_errors,
            fn_ty,
            "__call",
            self.call_loc,
        ) {
            // Calling a metamethod forwards `fnTy` as self.
            args_pack = unsafe {
                (*self.arena).add_type_pack_vector_type_id_optional_type_pack_id(
                    alloc::vec![fn_ty],
                    Some(args_pack),
                )
            };
            fn_ty = unsafe { follow_type_id(call_metamethod) };

            // Handle an overloaded __call metamethod.
            let it = unsafe { get_type_id::<IntersectionType>(fn_ty) };
            if !it.is_null() {
                let it = unsafe { &*it };
                let parts = it.parts.clone();
                for component in parts {
                    let component = unsafe { follow_type_id(component) };
                    result.metamethods.insert(component);
                    let fn_ptr = unsafe { get_type_id::<FunctionType>(component) };

                    if !fn_ptr.is_null()
                        && !self.is_arity_compatible(
                            args_pack,
                            unsafe { (*fn_ptr).arg_types },
                            self.builtin_types,
                        )
                    {
                        result.arity_mismatches.push(component);
                    } else {
                        self.test_function(result, component, args_pack, fn_location, unique_types);
                    }
                }
                return;
            }

            result.metamethods.insert(fn_ty);
        }

        // Handle non-metamethods and metamethods which aren't overloaded.
        self.test_function(result, fn_ty, args_pack, fn_location, unique_types);
    }
}
