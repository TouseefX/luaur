use crate::records::blocked_type::BlockedType;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct FindSimplificationBlockers {
    pub base: IterativeTypeVisitor,
    pub found: bool,
}

impl FindSimplificationBlockers {
    pub fn find_simplification_blockers(&mut self) {
        self.base.visit_once = true;
    }

    pub fn visit(&mut self, _ty: TypeId) -> bool {
        !self.found
    }

    pub fn visit_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
        self.found = true;
        false
    }

    pub fn visit_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
        self.found = true;
        false
    }

    pub fn visit_pending_expansion_type(
        &mut self,
        _ty: TypeId,
        _petv: &PendingExpansionType,
    ) -> bool {
        self.found = true;
        false
    }

    pub fn visit_function_type(&mut self, _ty: TypeId, _ftv: &FunctionType) -> bool {
        false
    }

    pub fn visit_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
        false
    }
}
