use crate::enums::pack_field::PackField;
use crate::records::generic_pack_mapping::GenericPackMapping;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::component::Component;
use crate::type_aliases::lookup_result::LookupResult;
use crate::type_aliases::path::Path;
use crate::type_aliases::type_pack_id::TypePackId;

impl Subtyping {
    pub fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_variadic_type_pack(
        &mut self,
        env: &mut SubtypingEnvironment,
        scope: *mut Scope,
        sub_tp: TypePackId,
        _sub: &GenericTypePack,
        super_tp: TypePackId,
        super_variadic: &VariadicTypePack,
    ) -> SubtypingResult {
        let any_type = unsafe { (*self.builtin_types).anyType };
        let unknown_type = unsafe { (*self.builtin_types).unknownType };
        let super_ty = super_variadic.ty;

        if super_ty == any_type || super_ty == unknown_type {
            return SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
        }

        let lookup_result = env.lookup_generic_pack(sub_tp);
        match lookup_result {
            LookupResult::V0(curr_mapping) => {
                let mut result = self
                    .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
                        env,
                        curr_mapping,
                        super_tp,
                        scope,
                    );
                result.with_sub_path(Path::from_components(alloc::vec![
                    Component::PackField(PackField::Tail),
                    Component::GenericPackMapping(GenericPackMapping {
                        mappedType: curr_mapping,
                    }),
                ]));
                result.with_super_component(Component::PackField(PackField::Tail));
                result
            }
            LookupResult::V1(_) => {
                let ok = env.mapped_generic_packs.bind_generic(sub_tp, super_tp);
                let mut result = SubtypingResult {
                    is_subtype: ok,
                    normalization_too_complex: false,
                    is_cacheable: false,
                    ..Default::default()
                };
                result.with_both_component(Component::PackField(PackField::Tail));
                result
            }
            LookupResult::V2(_) => {
                let mut result = SubtypingResult {
                    is_subtype: false,
                    normalization_too_complex: false,
                    is_cacheable: false,
                    ..Default::default()
                };
                result.with_both_component(Component::PackField(PackField::Tail));
                result
            }
        }
    }
}
