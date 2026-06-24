use crate::records::hash_subtype_constraint_record::HashSubtypeConstraintRecord;
use crate::records::subtype_constraint_record::SubtypeConstraintRecord;
use luaur_common::functions::hash_combine::hash_combine;

impl HashSubtypeConstraintRecord {
    pub fn operator_call(&self, c: &SubtypeConstraintRecord) -> usize {
        let mut result: usize = 0;
        hash_combine(&mut result, c.subTy as usize);
        hash_combine(&mut result, c.superTy as usize);
        hash_combine(&mut result, c.variance as usize);
        result
    }
}
