use crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch;

impl TypeInstantiationCountMismatch {
    #[inline]
    pub fn operator_eq(&self, rhs: &TypeInstantiationCountMismatch) -> bool {
        self.functionName == rhs.functionName
            && self.functionType == rhs.functionType
            && self.providedTypes == rhs.providedTypes
            && self.maximumTypes == rhs.maximumTypes
            && self.providedTypePacks == rhs.providedTypePacks
            && self.maximumTypePacks == rhs.maximumTypePacks
    }
}
