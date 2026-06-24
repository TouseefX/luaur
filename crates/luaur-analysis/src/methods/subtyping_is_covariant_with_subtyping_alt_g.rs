use crate::enums::variant::Variant;
use crate::records::index::Index;
use crate::records::intersection_type::IntersectionType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_iterator::TypeIterator;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_intersection_type_type_id_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_intersection: &IntersectionType,
        super_ty: TypeId,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = SubtypingResult::default();
        let mut i = 0usize;

        let mut it =
            TypeIterator::<IntersectionType>::type_iterator_type(sub_intersection as *const _);
        let end_it = TypeIterator::<IntersectionType>::type_iterator_default();
        while it.operator_ne(&end_it) {
            let ty = it.operator_deref();
            it.operator_inc();

            let mut candidate = self
                .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                    env, ty, super_ty, scope,
                );
            candidate.with_sub_component(Component::Index(Index {
                index: i,
                variant: Variant::Intersection,
            }));
            result.or_else(candidate);
            i += 1;

            if result.normalization_too_complex {
                return SubtypingResult {
                    is_subtype: false,
                    normalization_too_complex: true,
                    ..Default::default()
                };
            }
        }

        result
    }
}
