use crate::enums::type_field::TypeField;
use crate::records::metatable_type::MetatableType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::component::Component;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_metatable_type_metatable_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_mt: &MetatableType,
        super_mt: &MetatableType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            sub_mt.table(),
            super_mt.table(),
            scope,
        )
        .with_both_component(Component::TypeField(TypeField::Table))
        .and_also(
            self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                sub_mt.metatable(),
                super_mt.metatable(),
                scope,
            )
            .with_both_component(Component::TypeField(TypeField::Metatable))
            .to_owned(),
            crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
        )
        .to_owned()
    }
}
