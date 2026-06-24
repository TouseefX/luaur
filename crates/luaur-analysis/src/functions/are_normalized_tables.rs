use crate::functions::get_type_alt_j::get_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::table_type::TableType;
use crate::records::type_ids::TypeIds;

pub fn are_normalized_tables(tys: &TypeIds) -> bool {
    for ty in tys.order.iter() {
        unsafe {
            if !get_type_id::<TableType>(*ty).is_null()
                || !get_type_id::<MetatableType>(*ty).is_null()
            {
                continue;
            }

            if let Some(pt) = get_type_id::<PrimitiveType>(*ty).as_ref() {
                if pt.r#type == PrimitiveType::Table {
                    continue;
                }
            }

            return false;
        }
    }

    true
}
