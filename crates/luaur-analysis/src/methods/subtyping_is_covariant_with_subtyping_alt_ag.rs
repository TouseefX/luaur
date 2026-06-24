use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_type_function_instance_type_type_id_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_function_instance: &TypeFunctionInstanceType,
        super_ty: TypeId,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let (ty, mut errors) =
            self.handle_type_function_reduction_result(sub_function_instance, scope);

        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env, ty, super_ty, scope,
        )
        .with_errors(&mut errors)
        .with_sub_component(crate::type_aliases::component::Component::Reduction(
            crate::records::reduction::Reduction { resultType: ty },
        ))
        .to_owned()
    }
}
