use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveTypeConstraint {
    pub(crate) free_type: TypeId,
    pub(crate) expected_type: Option<TypeId>,
    pub(crate) primitive_type: TypeId,
}

#[allow(non_snake_case)]
impl PrimitiveTypeConstraint {
    pub fn freeType(&self) -> TypeId {
        self.free_type
    }

    pub fn expectedType(&self) -> Option<TypeId> {
        self.expected_type
    }

    pub fn primitiveType(&self) -> TypeId {
        self.primitive_type
    }
}
