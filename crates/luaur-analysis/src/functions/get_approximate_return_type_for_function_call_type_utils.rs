use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(
    mut ty: TypeId,
    seen: &mut DenseHashSet<TypeId>,
) -> Option<TypePackId> {
    ty = unsafe { follow_type_id(ty) };

    if seen.contains(&ty) {
        return None;
    }

    seen.insert(ty);

    let ftv = unsafe { get_type_id::<FunctionType>(ty) };
    if !ftv.is_null() {
        return Some(unsafe { (*ftv).ret_types });
    }

    let utv = unsafe { get_type_id::<UnionType>(ty) };
    if !utv.is_null() {
        let utv_ref = unsafe { &*utv };
        if !utv_ref.options.is_empty() {
            let first_ty = utv_ref.options[0];
            return get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(
                first_ty, seen,
            );
        }
    }

    None
}
