use crate::enums::variant::Variant;
use crate::records::index::Index;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_iterator::TypeIterator;
use crate::records::union_type::UnionType;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_type_id_union_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_ty: TypeId,
        super_union: &UnionType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = SubtypingResult {
            is_subtype: false,
            ..Default::default()
        };
        let mut index = 0usize;
        let mut it = TypeIterator::<UnionType>::type_iterator_type(super_union as *const UnionType);
        let end_it = TypeIterator::<UnionType>::type_iterator_default();
        while it.operator_ne(&end_it) {
            let ty = it.operator_deref();
            it.operator_inc();

            let mut next = self
                .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                    env, sub_ty, ty, scope,
                );
            if next.normalization_too_complex {
                return SubtypingResult {
                    is_subtype: false,
                    normalization_too_complex: true,
                    ..Default::default()
                };
            }
            if next.is_subtype {
                return next;
            }
            next.with_super_component(Component::Index(Index {
                index,
                variant: Variant::Union,
            }));
            result.and_also(
                next,
                crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
            );
            index += 1;
        }
        result
    }
}
