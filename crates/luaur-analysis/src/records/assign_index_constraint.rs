use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssignIndexConstraint {
    pub(crate) lhs_type: TypeId,
    pub(crate) index_type: TypeId,
    pub(crate) rhs_type: TypeId,
    /// The canonical write type of the property. It is _solely_ used to
    /// populate astTypes during constraint resolution. Nothing should ever
    /// block on it.
    pub(crate) prop_type: TypeId,
}

#[allow(non_snake_case)]
impl AssignIndexConstraint {
    pub fn lhsType(&self) -> TypeId {
        self.lhs_type
    }

    pub fn indexType(&self) -> TypeId {
        self.index_type
    }

    pub fn rhsType(&self) -> TypeId {
        self.rhs_type
    }

    pub fn propType(&self) -> TypeId {
        self.prop_type
    }
}
