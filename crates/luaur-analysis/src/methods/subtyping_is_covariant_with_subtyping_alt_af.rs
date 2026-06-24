use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::variadic_type_pack::VariadicTypePack;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_variadic_type_pack_variadic_type_pack_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_variadic: &VariadicTypePack,
        super_variadic: &VariadicTypePack,
        scope: *mut Scope,
    ) -> SubtypingResult {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            sub_variadic.ty,
            super_variadic.ty,
            scope,
        )
        .with_both_component(crate::type_aliases::component::Component::TypeField(
            crate::enums::type_field::TypeField::Variadic,
        ))
        .to_owned()
    }
}
