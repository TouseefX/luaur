use crate::records::table_skipping_visitor::TableSkippingVisitor;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl TableSkippingVisitor {
    pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tt: &TableType) -> bool {
        false
    }
}
