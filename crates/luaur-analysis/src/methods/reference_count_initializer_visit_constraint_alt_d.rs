use crate::enums::table_state::TableState;
use crate::records::reference_count_initializer::ReferenceCountInitializer;
use crate::records::table_type::TableType;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl ReferenceCountInitializer {
    pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
        if tt.state == TableState::Unsealed || tt.state == TableState::Free {
            unsafe {
                (*self.mutated_types).order.push(ty);
            }
        }

        true
    }
}
