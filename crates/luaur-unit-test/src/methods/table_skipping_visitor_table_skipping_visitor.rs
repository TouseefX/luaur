use crate::records::table_skipping_visitor::TableSkippingVisitor;
use luaur_analysis::records::iterative_type_visitor::IterativeTypeVisitor;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl TableSkippingVisitor {
    pub fn table_skipping_visitor_table_skipping_visitor() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        let mut base = IterativeTypeVisitor::default();
        base.iterative_type_visitor_string_bool_bool("TracingVisitor", true, true);

        Self {
            base,
            trace: alloc::vec::Vec::new(),
        }
    }

    pub fn run_type_id(&mut self, ty: TypeId) {
        luaur_analysis::records::iterative_type_visitor::IterativeTypeVisitorTrait::run_type_id(
            self, ty,
        );
    }
}

impl luaur_analysis::records::iterative_type_visitor::IterativeTypeVisitorTrait
    for TableSkippingVisitor
{
    fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
        &mut self.base
    }

    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        TableSkippingVisitor::visit_type_id(self, ty)
    }

    fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
        TableSkippingVisitor::visit_type_id_table_type(self, ty, tt)
    }
}
