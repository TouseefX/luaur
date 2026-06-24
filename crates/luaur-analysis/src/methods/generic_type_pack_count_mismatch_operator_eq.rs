use crate::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch;

impl GenericTypePackCountMismatch {
    #[inline]
    pub fn operator_eq(&self, rhs: &GenericTypePackCountMismatch) -> bool {
        self.subTyGenericPackCount == rhs.subTyGenericPackCount
            && self.superTyGenericPackCount == rhs.superTyGenericPackCount
    }
}
