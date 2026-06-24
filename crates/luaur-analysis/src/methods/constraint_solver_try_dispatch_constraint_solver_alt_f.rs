use crate::functions::follow_type::follow_type_id;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::infinite_type_finder::InfiniteTypeFinder;
use crate::records::intersection_type::IntersectionType;
use crate::records::iterative_type_visitor::IterativeTypeVisitorTrait;
use crate::records::metatable_type::MetatableType;
use crate::records::name_constraint::NameConstraint;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl ConstraintSolver {
    pub fn try_dispatch_name_constraint_not_null_constraint(
        &mut self,
        c: &NameConstraint,
        constraint: *const Constraint,
    ) -> bool {
        if self.is_blocked_type_id(c.named_type) {
            return self.block_type_id_not_null_constraint(c.named_type, constraint);
        }

        let target = unsafe { follow_type_id(c.named_type) };

        unsafe {
            if (*target).persistent || (*target).owning_arena != self.arena {
                return true;
            }
        }

        if let Some(tf) = unsafe {
            (*(*constraint).scope)
                .lookup_type(&crate::type_aliases::name_type::Name::from(c.name.as_str()))
        } {
            let signature = crate::records::instantiation_signature::InstantiationSignature {
                fn_sig: tf,
                arguments: c.type_parameters.clone(),
                pack_arguments: c.type_pack_parameters.clone(),
            };

            let mut itf = InfiniteTypeFinder::infinite_type_finder_infinite_type_finder(
                self,
                &signature,
                unsafe { core::ptr::NonNull::new_unchecked((*constraint).scope) },
            );
            itf.run_type_id(target);

            if itf.found_infinite_type {
                unsafe {
                    (*(*constraint).scope)
                        .invalid_type_aliases
                        .try_insert(c.name.clone(), (*constraint).location)
                };
                self.bind_not_null_constraint_type_id_type_id(constraint, target, unsafe {
                    (*self.builtin_types).errorType
                });
                return true;
            }
        }

        let ttv =
            unsafe { crate::functions::get_mutable_type::get_mutable_type_id::<TableType>(target) };
        if !ttv.is_null() {
            unsafe {
                if c.synthetic && (*ttv).name.is_none() {
                    (*ttv).synthetic_name = Some(c.name.clone());
                } else {
                    (*ttv).name = Some(c.name.clone());
                    (*ttv).instantiated_type_params = c.type_parameters.clone();
                    (*ttv).instantiated_type_pack_params = c.type_pack_parameters.clone();
                }
            }
        } else {
            let mtv = unsafe {
                crate::functions::get_mutable_type::get_mutable_type_id::<MetatableType>(target)
            };
            if !mtv.is_null() {
                unsafe { (*mtv).syntheticName = Some(c.name.clone()) };
            }
        }

        true
    }
}
