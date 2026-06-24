use crate::records::subtype_constraint_record::SubtypeConstraintRecord;

impl SubtypeConstraintRecord {
    pub fn operator_eq(&self, other: &SubtypeConstraintRecord) -> bool {
        self.subTy == other.subTy
            && self.superTy == other.superTy
            && self.variance == other.variance
    }
}
