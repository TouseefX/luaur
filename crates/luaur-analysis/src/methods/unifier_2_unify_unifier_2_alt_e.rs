use crate::enums::unify_result::UnifyResult;
use crate::functions::are_compatible::are_compatible;
use crate::records::unifier_2::Unifier2;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl Unifier2 {
    pub fn unify_union_type_type_id(
        &mut self,
        sub_union: &UnionType,
        super_ty: TypeId,
    ) -> UnifyResult {
        let mut result = UnifyResult::Ok;

        for sub_option in sub_union.options.iter() {
            if unsafe { are_compatible(*sub_option, super_ty) } {
                result &= self.unify_type_id_type_id(*sub_option, super_ty);
            }
        }

        result
    }
}
