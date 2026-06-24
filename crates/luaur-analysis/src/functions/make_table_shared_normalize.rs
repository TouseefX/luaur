use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn make_table_shared_type_id_dense_hash_set_type_id(
    ty: TypeId,
    seen: &mut DenseHashSet<TypeId>,
) {
    let mut ty = unsafe { follow_type_id(ty) };

    if seen.contains(&ty) {
        return;
    }

    seen.insert(ty);

    let table_ty_ptr = unsafe { get_mutable_type_id::<TableType>(ty) };
    if !table_ty_ptr.is_null() {
        let table_ty = unsafe { &mut *table_ty_ptr };
        for (_, prop) in &mut table_ty.props {
            if prop.write_ty.is_some() {
                prop.write_ty = prop.read_ty;
            }
        }
        return;
    }

    let metatable_ty_ptr = unsafe { get_type_id::<MetatableType>(ty) };
    if !metatable_ty_ptr.is_null() {
        let metatable_ty = unsafe { &*metatable_ty_ptr };
        make_table_shared_type_id_dense_hash_set_type_id(metatable_ty.metatable(), seen);
        make_table_shared_type_id_dense_hash_set_type_id(metatable_ty.table(), seen);
    }
}
