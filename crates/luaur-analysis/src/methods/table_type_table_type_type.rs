use crate::enums::table_state::TableState;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;

impl TableType {
    pub fn table_type() -> Self {
        Self::table_type_table_state_type_level_scope(
            TableState::Unsealed,
            TypeLevel::default(),
            core::ptr::null_mut(),
        )
    }
}
