//! Source: `Analysis/src/Normalize.cpp:875-923` (hand-ported)
//!
//! Two `isCacheable` overloads from Normalize.cpp:
//!   * `isCacheable(TypePackId, Set<TypeId>&)`  (Normalize.cpp:875-894)
//!   * `isCacheable(TypeId, Set<TypeId>&)`       (Normalize.cpp:896-923)
use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn is_cacheable_type_pack_id_set_type_id(
    tp: TypePackId,
    seen: &mut DenseHashSet<TypeId>,
) -> bool {
    let tp = unsafe { crate::functions::follow_type_pack::follow_type_pack_id(tp) };

    unsafe {
        let mut it = begin_type_pack_id(tp);
        let end_it = end(tp);

        while it.operator_ne(&end_it) {
            if !is_cacheable(*it.operator_deref(), seen) {
                return false;
            }
            it.operator_inc();
        }

        if let Some(tail) = it.tail() {
            if !get_type_pack_id::<FreeTypePack>(tail).is_null()
                || !get_type_pack_id::<BlockedTypePack>(tail).is_null()
                || !get_type_pack_id::<TypeFunctionInstanceTypePack>(tail).is_null()
            {
                return false;
            }
        }
    }

    true
}

pub fn is_cacheable(ty: TypeId, seen: &mut DenseHashSet<TypeId>) -> bool {
    if seen.contains(&ty) {
        return true;
    }
    seen.insert(ty);

    let ty = unsafe { follow_type_id(ty) };

    if !unsafe { get_type_id::<FreeType>(ty).is_null() }
        || !unsafe { get_type_id::<BlockedType>(ty).is_null() }
        || !unsafe { get_type_id::<PendingExpansionType>(ty).is_null() }
    {
        return false;
    }

    if let Some(tfi) = unsafe { get_type_id::<TypeFunctionInstanceType>(ty).as_ref() } {
        for t in &tfi.type_arguments {
            if !is_cacheable(*t, seen) {
                return false;
            }
        }
        for tp in &tfi.pack_arguments {
            if !is_cacheable_type_pack_id_set_type_id(*tp, seen) {
                return false;
            }
        }
    }

    true
}
