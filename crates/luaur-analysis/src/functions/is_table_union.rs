use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

pub fn is_table_union(ty: TypeId) -> bool {
    unsafe {
        let followed = follow_type_id(ty);
        let ut = get::<UnionType>(followed);
        if ut.is_null() {
            return false;
        }

        let ut = &*ut;
        for &option in &ut.options {
            if get::<TableType>(follow_type_id(option)).is_null() {
                return false;
            }
        }

        true
    }
}
