use crate::enums::pack_field::PackField;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl Subtyping {
    pub fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_variadic_type_pack(
        &mut self,
        env: &mut SubtypingEnvironment,
        scope: *mut Scope,
        _sub_tp: TypePackId,
        sub: &VariadicTypePack,
        _super_tp: TypePackId,
        super_variadic: &VariadicTypePack,
    ) -> SubtypingResult {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            sub.ty,
            super_variadic.ty,
            scope,
        )
        .with_both_component(crate::type_aliases::component::Component::TypeField(
            crate::enums::type_field::TypeField::Variadic,
        ))
        .with_both_component(crate::type_aliases::component::Component::PackField(
            PackField::Tail,
        ))
        .to_owned()
    }
}
