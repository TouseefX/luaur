use crate::records::reduction::Reduction;

impl Reduction {
    pub fn operator_eq(&self, other: &Reduction) -> bool {
        self.resultType == other.resultType
    }
}
