use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_string::is_string;
use crate::records::extern_type::ExternType;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::table_type::TableType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn compute_keys_of(
    ty: TypeId,
    result: &mut BTreeSet<alloc::string::String>,
    seen: &mut DenseHashSet<TypeId>,
    is_raw: bool,
    ctx: *mut TypeFunctionContext,
) -> bool {
    let ctx_ref = unsafe { &*ctx };
    let ty = unsafe { follow_type_id(ty) };

    if unsafe { !get_type_id::<PrimitiveType>(ty).is_null() } {
        return false;
    }

    if seen.contains(&ty) {
        return true;
    }
    seen.insert(ty);

    if let Some(table_ty) = unsafe { get_type_id::<TableType>(ty).as_ref() } {
        if let Some(indexer) = table_ty.indexer {
            if is_string(indexer.index_type) {
                return false;
            }
        }

        for key in table_ty.props.keys() {
            result.insert(key.clone());
        }
        return true;
    }

    if let Some(metatable_ty) = unsafe { get_type_id::<MetatableType>(ty).as_ref() } {
        let mut res = true;

        if !is_raw {
            let mut dummy = Vec::new();
            if let Some(mm_type) = find_metatable_entry(
                ctx_ref.builtins.as_ptr(),
                &mut dummy,
                ty,
                "__index",
                Location::default(),
            ) {
                res = res && compute_keys_of(mm_type, result, seen, is_raw, ctx);
            }
        }

        res = res && compute_keys_of(metatable_ty.table(), result, seen, is_raw, ctx);
        return res;
    }

    if let Some(extern_ty) = unsafe { get_type_id::<ExternType>(ty).as_ref() } {
        for key in extern_ty.props.keys() {
            result.insert(key.clone());
        }

        let mut res = true;
        if extern_ty.metatable.is_some() && !is_raw {
            let mut dummy = Vec::new();
            if let Some(mm_type) = find_metatable_entry(
                ctx_ref.builtins.as_ptr(),
                &mut dummy,
                ty,
                "__index",
                Location::default(),
            ) {
                res = res && compute_keys_of(mm_type, result, seen, is_raw, ctx);
            }
        }

        if let Some(parent) = extern_ty.parent {
            res = res && compute_keys_of(parent, result, seen, is_raw, ctx);
        }

        return res;
    }

    LUAU_ASSERT!(false);
    false
}
