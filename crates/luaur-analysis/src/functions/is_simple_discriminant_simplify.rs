use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_approximately_falsy_type::is_approximately_falsy_type;
use crate::functions::is_approximately_truthy_type::is_approximately_truthy_type;
use crate::records::extern_type::ExternType;
use crate::records::negation_type::NegationType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn is_simple_discriminant(ty: TypeId, seen: &mut DenseHashSet<TypeId>) -> bool {
    let mut ty = unsafe { follow_type_id(ty) };
    if seen.contains(&ty) {
        return false;
    }
    seen.insert(ty);

    let ttv_ptr = unsafe { get_type_id::<TableType>(ty) };
    if !ttv_ptr.is_null() {
        let ttv = unsafe { &*ttv_ptr };
        if ttv.props.len() == 1 && ttv.indexer.is_none() {
            let prop = ttv.props.values().next().unwrap();
            let read_ok = prop
                .read_ty
                .map_or(true, |rt| is_simple_discriminant(rt, seen));
            let write_ok = prop
                .write_ty
                .map_or(true, |wt| is_simple_discriminant(wt, seen));
            return read_ok && write_ok;
        }
    }

    let nt_ptr = unsafe { get_type_id::<NegationType>(ty) };
    if !nt_ptr.is_null() {
        return is_simple_discriminant(unsafe { (*nt_ptr).ty }, seen);
    }

    let is_prim = !unsafe { get_type_id::<PrimitiveType>(ty) }.is_null();
    let is_sing = !unsafe { get_type_id::<SingletonType>(ty) }.is_null();
    let is_ext = !unsafe { get_type_id::<ExternType>(ty) }.is_null();

    is_prim
        || is_sing
        || is_ext
        || is_approximately_truthy_type(ty)
        || is_approximately_falsy_type(ty)
}
