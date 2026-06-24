use crate::records::negation_type::NegationType;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct NegationTypeFinder {
    pub base: TypeOnceVisitor,
    pub found: bool,
}

impl NegationTypeFinder {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("NegationTypeFinder"), false),
            found: false,
        }
    }

    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        !self.found
    }

    pub fn visit_negation_type(&mut self, _ty: TypeId, _ntv: &NegationType) -> bool {
        self.found = true;
        !self.found
    }
}
