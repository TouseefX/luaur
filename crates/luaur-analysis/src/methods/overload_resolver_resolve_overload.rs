//! Node: `cxx:Method:Luau.Analysis:Analysis/src/OverloadResolver.cpp:175:overload_resolver_resolve_overload`
//! Source: `Analysis/src/OverloadResolver.cpp:175-196` (hand-ported)

use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::overload_resolution::OverloadResolution;
use crate::records::overload_resolver::OverloadResolver;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl OverloadResolver {
    /// C++ `OverloadResolution resolveOverload(TypeId ty, TypePackId argsPack, Location fnLocation, NotNull<DenseHashSet<TypeId>> uniqueTypes, bool useFreeTypeBounds)`.
    pub fn resolve_overload(
        &mut self,
        ty: TypeId,
        args_pack: TypePackId,
        fn_location: Location,
        unique_types: *mut DenseHashSet<TypeId>,
        _use_free_type_bounds: bool,
    ) -> OverloadResolution {
        let mut result = OverloadResolution {
            ok: alloc::vec::Vec::new(),
            non_functions: alloc::vec::Vec::new(),
            potential_overloads: alloc::vec::Vec::new(),
            incompatible_overloads: alloc::vec::Vec::new(),
            arity_mismatches: alloc::vec::Vec::new(),
            metamethods: DenseHashSet::new(core::ptr::null_mut()),
        };

        let ty = unsafe { follow_type_id(ty) };

        let it = unsafe { get_type_id::<IntersectionType>(ty) };
        if !it.is_null() {
            let parts = unsafe { (*it).parts.clone() };
            for component in parts {
                self.test_function_or_union(
                    &mut result,
                    component,
                    args_pack,
                    fn_location,
                    unique_types,
                );
            }
        } else {
            self.test_function_or_union(&mut result, ty, args_pack, fn_location, unique_types);
        }

        result
    }
}
