use crate::enums::table_state::TableState;
use crate::records::generic_type_finder::GenericTypeFinder;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl GenericTypeFinder {
    pub fn visit_type_id_luau_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
        // Assuming TableType has a state field or accessor
        // if ttv.state == TableState::Generic { self.found = true; }
        !self.found
    }
}
