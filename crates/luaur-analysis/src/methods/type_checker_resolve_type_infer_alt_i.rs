use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::functions::is_undecidable::is_undecidable;
use crate::functions::maybe_singleton::maybe_singleton;
use crate::records::eq_predicate::EqPredicate;
use crate::records::singleton_type::SingletonType;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;

impl TypeChecker {
    pub fn resolve_eq_predicate_refinement_map_scope_ptr_bool(
        &mut self,
        eq_p: &EqPredicate,
        refis: &mut RefinementMap,
        scope: ScopePtr,
        sense: bool,
    ) {
        let rhs = unsafe {
            let ty = follow_type_id(eq_p.ty);
            let union = get_type_id::<UnionType>(ty);
            if !union.is_null() {
                (*union).options.clone()
            } else {
                alloc::vec![ty]
            }
        };

        if sense && rhs.iter().copied().any(is_undecidable) {
            return;
        }

        let checker = self as *mut TypeChecker;
        let eq_ty = eq_p.ty;
        let location = eq_p.location;
        let scope_for_predicate = scope.clone();

        let predicate = Box::new(move |option: TypeId| -> Option<TypeId> {
            if !sense && is_nil(eq_ty) {
                return if is_undecidable(option) || !is_nil(option) {
                    Some(option)
                } else {
                    None
                };
            }

            if maybe_singleton(eq_ty) {
                let option_is_subtype = unsafe {
                    (*checker)
                        .can_unify_type_id_type_id_scope_ptr_location(
                            option,
                            eq_ty,
                            &scope_for_predicate,
                            &location,
                        )
                        .is_empty()
                };
                let target_is_subtype = unsafe {
                    (*checker)
                        .can_unify_type_id_type_id_scope_ptr_location(
                            eq_ty,
                            option,
                            &scope_for_predicate,
                            &location,
                        )
                        .is_empty()
                };

                if sense {
                    if option_is_subtype && !target_is_subtype {
                        return Some(option);
                    } else if !option_is_subtype && target_is_subtype {
                        return Some(unsafe { follow_type_id(eq_ty) });
                    } else if !option_is_subtype && !target_is_subtype {
                        return None;
                    } else if option_is_subtype && target_is_subtype {
                        return Some(unsafe { follow_type_id(eq_ty) });
                    }
                } else {
                    let is_option_singleton =
                        unsafe { !get_type_id::<SingletonType>(option).is_null() };
                    if !is_option_singleton {
                        return Some(option);
                    } else if option_is_subtype && target_is_subtype {
                        return None;
                    }
                }
            }

            Some(option)
        });

        self.refine_l_value(&eq_p.lvalue, refis, scope, predicate);
    }
}
