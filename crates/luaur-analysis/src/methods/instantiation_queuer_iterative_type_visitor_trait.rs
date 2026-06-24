use crate::records::extern_type::ExternType;
use crate::records::instantiation_queuer::InstantiationQueuer;
use crate::records::iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait};
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitorTrait for InstantiationQueuer {
    fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
        &mut self.base
    }

    fn visit_type_id_pending_expansion_type(
        &mut self,
        ty: TypeId,
        petv: &PendingExpansionType,
    ) -> bool {
        InstantiationQueuer::visit_type_id_pending_expansion_type(self, ty, petv)
    }

    fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        tfit: &TypeFunctionInstanceType,
    ) -> bool {
        InstantiationQueuer::visit_type_id_type_function_instance_type(self, ty, tfit)
    }

    fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
        InstantiationQueuer::visit_type_id_extern_type(self, ty, etv)
    }
}
