use crate::enums::unify_result::UnifyResult;

#[allow(non_snake_case)]
pub fn operator_bitand(lhs: UnifyResult, rhs: UnifyResult) -> UnifyResult {
    if lhs == UnifyResult::Ok {
        return rhs;
    }
    lhs
}

impl core::ops::BitAnd for UnifyResult {
    type Output = UnifyResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        operator_bitand(self, rhs)
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitand as operator_bitand_unify_result_unify_result;
