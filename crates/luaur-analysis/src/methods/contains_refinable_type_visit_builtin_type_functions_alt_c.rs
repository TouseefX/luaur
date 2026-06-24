use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _table: &TableType) -> bool {
        !self.found
    }
}
