use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_prim::is_prim;
use crate::functions::is_string::is_string;
use crate::records::any_type::AnyType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FInt;

pub fn has_length(ty: TypeId, seen: &mut DenseHashSet<TypeId>, recursion_count: &mut i32) -> bool {
    let mut _rl = RecursionLimiter {
        base: unsafe { core::mem::zeroed() },
        native_stack_guard: unsafe { core::mem::zeroed() },
    };
    _rl.recursion_limiter_recursion_limiter(
        "Type::hasLength",
        recursion_count as *mut i32 as *mut core::ffi::c_int,
        FInt::LuauTypeInferRecursionLimit.get() as core::ffi::c_int,
    );

    let ty = unsafe { follow_type_id(ty) };

    if seen.contains(&ty) {
        return true;
    }

    unsafe {
        if is_string(ty)
            || is_prim(ty, PrimitiveType::Table)
            || !get_type_id::<AnyType>(ty).is_null()
            || !get_type_id::<TableType>(ty).is_null()
            || !get_type_id::<MetatableType>(ty).is_null()
        {
            return true;
        }

        let uty = get_type_id::<UnionType>(ty);
        if !uty.is_null() {
            seen.insert(ty);

            for &part in &(*uty).options {
                if !has_length(part, seen, recursion_count) {
                    return false;
                }
            }

            return true;
        }

        let ity = get_type_id::<IntersectionType>(ty);
        if !ity.is_null() {
            seen.insert(ty);

            for &part in &(*ity).parts {
                if has_length(part, seen, recursion_count) {
                    return true;
                }
            }

            return false;
        }
    }

    false
}
