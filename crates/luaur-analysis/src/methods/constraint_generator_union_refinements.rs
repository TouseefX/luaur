use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::intersection_type::IntersectionType;
use crate::records::refinement_partition::RefinementPartition;
use crate::records::scope::Scope;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::refinement_context::RefinementContext;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn union_refinements(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        lhs: &RefinementContext,
        rhs: &RefinementContext,
        dest: *mut RefinementContext,
        _constraints: *mut Vec<ConstraintV>,
    ) {
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

        for (def, partition) in lhs.iter() {
            let rhs_partition = match rhs.get(def) {
                Some(p) => p,
                None => continue,
            };

            LUAU_ASSERT!(!partition.discriminant_types.is_empty());
            LUAU_ASSERT!(!rhs_partition.discriminant_types.is_empty());

            // C++ `intersect(types)`: 1 -> the sole type, 2 -> makeIntersect, more -> an IntersectionType.
            let left_discriminant_ty = {
                let types = &partition.discriminant_types;
                if types.len() == 1 {
                    types[0]
                } else if types.len() == 2 {
                    self.make_intersect(scope, location, types[0], types[1])
                } else {
                    unsafe {
                        (*self.arena).add_type(IntersectionType {
                            parts: types.clone(),
                        })
                    }
                }
            };

            let right_discriminant_ty = {
                let types = &rhs_partition.discriminant_types;
                if types.len() == 1 {
                    types[0]
                } else if types.len() == 2 {
                    self.make_intersect(scope, location, types[0], types[1])
                } else {
                    unsafe {
                        (*self.arena).add_type(IntersectionType {
                            parts: types.clone(),
                        })
                    }
                }
            };

            let union_ty = self.make_union_scope_ptr_location_type_id_type_id(
                scope_raw,
                location,
                left_discriminant_ty,
                right_discriminant_ty,
            );

            let should_append_nil =
                partition.should_append_nil_type || rhs_partition.should_append_nil_type;

            unsafe {
                (*dest).insert(*def, RefinementPartition::default());
                let dest_partition = (*dest).get_mut(def).unwrap();
                dest_partition.discriminant_types.push(union_ty);
                dest_partition.should_append_nil_type |= should_append_nil;
            }
        }
    }
}
