use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::try_pair::TryPair;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_try_pair_sub_ty_super_ty_not_null_scope<
        SubTy,
        SuperTy,
    >(
        &mut self,
        env: &mut SubtypingEnvironment,
        pair: &TryPair<*const SubTy, *const SuperTy>,
        scope: *mut Scope,
    ) -> SubtypingResult {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            pair.first as TypeId,
            pair.second as TypeId,
            scope,
        )
    }
}
