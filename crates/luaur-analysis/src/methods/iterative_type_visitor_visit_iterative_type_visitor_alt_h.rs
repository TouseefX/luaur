use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_table_type(&mut self, ty: TypeId, _ttv: &TableType) -> bool {
        self.visit_type_id(ty)
    }
}
