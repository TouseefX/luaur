use crate::records::generic_type_definition::GenericTypeDefinition;

impl GenericTypeDefinition {
    #[inline]
    pub fn operator_eq(&self, rhs: &GenericTypeDefinition) -> bool {
        self.ty == rhs.ty && self.defaultValue == rhs.defaultValue
    }
}
