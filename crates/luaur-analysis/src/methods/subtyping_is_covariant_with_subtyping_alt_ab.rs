use crate::functions::is_subtype_normalized_string::is_subtype_normalized_string;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_normalized_string_type_normalized_string_type_not_null_scope(
        &mut self,
        _env: &mut SubtypingEnvironment,
        sub_string: &NormalizedStringType,
        super_string: &NormalizedStringType,
        _scope: *mut Scope,
    ) -> SubtypingResult {
        SubtypingResult {
            is_subtype: is_subtype_normalized_string(sub_string, super_string),
            ..Default::default()
        }
    }
}
