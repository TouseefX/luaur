use crate::enums::reason::Reason;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CannotAssignToNever {
    /// type of the rvalue being assigned
    pub(crate) rhsType: TypeId,
    /// Originating type.
    pub(crate) cause: Vec<TypeId>,
    pub(crate) reason: Reason,
}

impl CannotAssignToNever {
    pub const fn new(rhs_type: TypeId, cause: Vec<TypeId>, reason: Reason) -> Self {
        Self {
            rhsType: rhs_type,
            cause,
            reason,
        }
    }
}

#[allow(non_snake_case)]
impl CannotAssignToNever {
    pub fn rhsType(&self) -> TypeId {
        self.rhsType
    }

    pub fn cause(&self) -> &[TypeId] {
        &self.cause
    }

    pub fn reason(&self) -> Reason {
        self.reason
    }
}
