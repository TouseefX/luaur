use crate::records::blocked_type::BlockedType;
use crate::records::extern_type::ExternType;
use crate::records::find_simplification_blockers::FindSimplificationBlockers;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait};
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitorTrait for FindSimplificationBlockers {
    fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
        &mut self.base
    }

    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        FindSimplificationBlockers::visit_type_id(self, ty)
    }

    fn visit_type_id_blocked_type(&mut self, ty: TypeId, btv: &BlockedType) -> bool {
        FindSimplificationBlockers::visit_type_id_blocked_type(self, ty, btv)
    }

    fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
        FindSimplificationBlockers::visit_type_id_free_type(self, ty, ftv)
    }

    fn visit_type_id_pending_expansion_type(
        &mut self,
        ty: TypeId,
        petv: &PendingExpansionType,
    ) -> bool {
        FindSimplificationBlockers::visit_type_id_pending_expansion_type(self, ty, petv)
    }

    fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
        FindSimplificationBlockers::visit_type_id_function_type(self, ty, ftv)
    }

    fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
        FindSimplificationBlockers::visit_type_id_extern_type(self, ty, etv)
    }
}
