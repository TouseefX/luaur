use crate::functions::flatten_intersection::flatten_intersection;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get;
use crate::records::intersection_type::IntersectionType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn is_table_intersection(ty: TypeId) -> bool {
    unsafe {
        if get::<IntersectionType>(follow_type_id(ty)).is_null() {
            return false;
        }

        let parts = flatten_intersection(ty);
        parts.iter().all(|&part| !get::<TableType>(part).is_null())
    }
}
