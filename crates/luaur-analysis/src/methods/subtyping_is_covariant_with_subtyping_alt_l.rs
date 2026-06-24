use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_singleton_type_singleton_type_not_null_scope(
        &mut self,
        _env: &mut SubtypingEnvironment,
        sub_singleton: &SingletonType,
        super_singleton: &SingletonType,
        _scope: *mut Scope,
    ) -> SubtypingResult {
        SubtypingResult {
            is_subtype: sub_singleton.operator_eq(super_singleton),
            ..Default::default()
        }
    }
}
