//! Source: `Analysis/src/Unifier.cpp` (Unifier::occursCheck(TypeId,...), L2606-2642)
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::r#type::Type;
use crate::records::unifier::Unifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

impl Unifier {
    /// `bool Unifier::occursCheck(TypeId needle, TypeId haystack, bool reversed)`
    pub fn occurs_check_type_id_type_id_bool(
        &mut self,
        needle: TypeId,
        haystack: TypeId,
        reversed: bool,
    ) -> bool {
        let shared_state = unsafe { &mut *self.shared_state };
        shared_state.temp_seen_ty.clear();

        let occurs = self.occurs_check_dense_hash_set_type_id_type_id_type_id(
            &mut shared_state.temp_seen_ty,
            needle,
            haystack,
        );

        if occurs {
            let mut inner_state = self.unifier_make_child_unifier();
            let ut = unsafe { get_type_id::<UnionType>(haystack) };
            let it = unsafe { get_type_id::<IntersectionType>(haystack) };

            if !ut.is_null() {
                if reversed {
                    inner_state.unifier_try_unify_union_with_type(haystack, ut, needle);
                } else {
                    inner_state
                        .unifier_try_unify_type_with_union(needle, haystack, ut, false, false);
                }
            } else if !it.is_null() {
                if reversed {
                    inner_state.unifier_try_unify_intersection_with_type(
                        haystack, it, needle, false, false,
                    );
                } else {
                    inner_state.unifier_try_unify_type_with_intersection(needle, haystack, it);
                }
            } else {
                inner_state.failure = true;
            }

            if inner_state.failure {
                self.report_error_location_type_error_data(
                    self.location,
                    TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
                );
                // C++: log.replace(needle, BoundType{builtinTypes->errorType});
                let error_ty = unsafe { (*self.builtin_types).errorType };
                self.log
                    .replace_type_id_t(needle, Type::new(TypeVariant::Bound(error_ty)));
            }
        }

        occurs
    }
}
