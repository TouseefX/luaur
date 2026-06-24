use crate::records::normalized_type::NormalizedType;
use crate::records::primitive_type::PrimitiveType;

impl NormalizedType {
    pub fn has_top_table(&self) -> bool {
        if !self.has_tables() {
            return false;
        }

        for &ty in &self.tables.order {
            let prim_ty =
                unsafe { crate::functions::get_type_alt_j::get_type_id::<PrimitiveType>(ty) };
            if !prim_ty.is_null() {
                let prim_ref = unsafe { &*prim_ty };
                if prim_ref.r#type == PrimitiveType::Table {
                    return true;
                }
            }
        }

        false
    }
}
