use crate::records::blocked_type::BlockedType;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use core::option::Option;

#[derive(Debug, Clone)]
pub struct BlockedTypeFinder {
    pub base: TypeOnceVisitor,
    pub blocked: Option<TypeId>,
}

impl BlockedTypeFinder {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("ContainsGenerics_DEPRECATED"), true),
            blocked: None,
        }
    }

    pub fn visit(&mut self, _ty: TypeId) -> bool {
        self.blocked.is_none()
    }

    pub fn visit_blocked(&mut self, ty: TypeId, _bt: &BlockedType) -> bool {
        self.blocked = Some(ty);
        false
    }
}
