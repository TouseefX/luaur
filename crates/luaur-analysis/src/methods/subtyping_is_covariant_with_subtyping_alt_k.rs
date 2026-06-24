use crate::functions::get_singleton_type::get_singleton_type;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::primitive_type::PrimitiveType;
use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_singleton_type_primitive_type_not_null_scope(
        &mut self,
        _env: &mut SubtypingEnvironment,
        sub_singleton: &SingletonType,
        super_prim: &PrimitiveType,
        _scope: *mut Scope,
    ) -> SubtypingResult {
        if !get_singleton_type::<StringSingleton>(sub_singleton as *const SingletonType).is_null() {
            if super_prim.r#type == PrimitiveType::String {
                return SubtypingResult {
                    is_subtype: true,
                    ..Default::default()
                };
            }
        }

        if !get_singleton_type::<BooleanSingleton>(sub_singleton as *const SingletonType).is_null()
        {
            if super_prim.r#type == PrimitiveType::Boolean {
                return SubtypingResult {
                    is_subtype: true,
                    ..Default::default()
                };
            }
        }

        SubtypingResult {
            is_subtype: false,
            ..Default::default()
        }
    }
}
