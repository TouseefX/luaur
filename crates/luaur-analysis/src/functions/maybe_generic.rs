use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_generic::is_generic;
use crate::records::free_type::FreeType;
use crate::records::intersection_type::IntersectionType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

pub fn maybe_generic(ty: TypeId) -> bool {
    LUAU_ASSERT!(!FFlag::LuauInstantiateInSubtyping.get());

    let ty = unsafe { follow_type_id(ty) };

    if unsafe { !get_type_id::<FreeType>(ty).is_null() } {
        return true;
    }

    if unsafe { !get_type_id::<TableType>(ty).is_null() } {
        // TODO: recurse on table types CLI-39914
        return true;
    }

    if let Some(itv) = unsafe { get_type_id::<IntersectionType>(ty).as_ref() } {
        return itv.parts.iter().any(|&part| maybe_generic(part));
    }

    is_generic(ty)
}
