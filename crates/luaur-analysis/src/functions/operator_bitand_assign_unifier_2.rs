use crate::enums::unify_result::UnifyResult;

#[allow(non_snake_case)]
pub fn operator_bitand_assign(lhs: &mut UnifyResult, rhs: UnifyResult) -> &mut UnifyResult {
    if *lhs == UnifyResult::Ok {
        *lhs = rhs;
    }
    lhs
}

impl core::ops::BitAndAssign for UnifyResult {
    fn bitand_assign(&mut self, rhs: Self) {
        operator_bitand_assign(self, rhs);
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitand_assign as operator_bitand_assign_unify_result_unify_result;
