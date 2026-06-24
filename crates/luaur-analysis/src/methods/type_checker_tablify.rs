//! @interface-stub
use crate::enums::table_state::TableState;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::free_type::FreeType;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

impl TypeChecker {
    pub fn tablify(&mut self, ty: TypeId) {
        unsafe {
            let ty = follow_type_id(ty);
            let free = get_type_id::<FreeType>(ty);

            if !free.is_null() {
                (*as_mutable_type_id(ty)).ty =
                    TypeVariant::Table(TableType::table_type_table_state_type_level_scope(
                        TableState::Free,
                        (*free).level,
                        core::ptr::null_mut(),
                    ));
            }
        }
    }
}
