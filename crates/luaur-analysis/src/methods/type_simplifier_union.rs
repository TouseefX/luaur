use crate::enums::relation::Relation;
use crate::enums::table_state::TableState;
use crate::functions::begin_type::begin_union_type;
use crate::functions::end_type::end_union_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::never_type::NeverType;
use crate::records::property_type::Property;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn union_(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let mut rl = RecursionLimiter {
            base: unsafe { core::mem::zeroed() },
            native_stack_guard: unsafe { core::mem::zeroed() },
        };
        rl.recursion_limiter_recursion_limiter(
            "TypeSimplifier::union",
            &mut self.recursion_depth,
            15,
        );

        let left = self.simplify_type_id(left);
        let right = self.simplify_type_id(right);
        if unsafe { !get_type_id::<NeverType>(left).is_null() } {
            return right;
        }
        if unsafe { !get_type_id::<NeverType>(right).is_null() } {
            return left;
        }
        if let Some(left_union) = unsafe { get_type_id::<UnionType>(left).as_ref() } {
            let mut changed = false;
            let mut ub = crate::records::union_builder::UnionBuilder::union_builder(
                self.arena as *mut _,
                self.builtin_types as *mut _,
            );
            ub.reserve(left_union.options.len());

            let mut iter = begin_union_type(left_union);
            let end = end_union_type(left_union);
            while iter.operator_ne(&end) {
                let part = iter.operator_deref();

                if unsafe { !get_type_id::<NeverType>(part).is_null() } {
                    changed = true;
                    iter.operator_inc();
                    continue;
                }

                match relate_type_id_type_id(part, right) {
                    Relation::Coincident | Relation::Superset => return left,
                    Relation::Subset => {
                        ub.add(right);
                        changed = true;
                    }
                    _ => {
                        ub.add(part);
                        ub.add(right);
                        changed = true;
                    }
                }

                iter.operator_inc();
            }

            if !changed {
                return left;
            }

            if ub.size() == 0 {
                return right;
            }

            return ub.build();
        }
        if unsafe { !get_type_id::<UnionType>(right).is_null() } {
            return self.union_(right, left);
        }

        let relation = relate_type_id_type_id(left, right);
        if left == right || relation == Relation::Coincident || relation == Relation::Superset {
            return left;
        }

        if relation == Relation::Subset {
            return right;
        }

        if let Some(left_singleton) = unsafe { get_type_id::<SingletonType>(left).as_ref() } {
            if let Some(left_bool) = left_singleton.variant.get_if_0() {
                if let Some(right_singleton) =
                    unsafe { get_type_id::<SingletonType>(right).as_ref() }
                {
                    if let Some(right_bool) = right_singleton.variant.get_if_0() {
                        if left_bool.value != right_bool.value {
                            return unsafe { (*self.builtin_types).booleanType };
                        }
                    }
                }
            }
        }

        let left_table = unsafe { get_type_id::<TableType>(left).as_ref() };
        let right_table = unsafe { get_type_id::<TableType>(right).as_ref() };
        if let (Some(left_table), Some(right_table)) = (left_table, right_table) {
            if left_table.props.len() == 1 && right_table.props.len() == 1 {
                let (prop_name, left_prop) = left_table.props.iter().next().unwrap();
                let (right_prop_name, right_prop) = right_table.props.iter().next().unwrap();

                if right_prop_name != prop_name {
                    return unsafe {
                        (*self.arena.cast_mut()).add_type(UnionType {
                            options: alloc::vec![left, right],
                        })
                    };
                }

                if !left_prop.is_read_only()
                    || !right_prop.is_read_only()
                    || left_table.state != TableState::Sealed
                    || right_table.state != TableState::Sealed
                {
                    return unsafe {
                        (*self.arena.cast_mut()).add_type(UnionType {
                            options: alloc::vec![left, right],
                        })
                    };
                }

                let left_read_ty = left_prop.read_ty.expect("read-only property has read type");
                let right_read_ty = right_prop
                    .read_ty
                    .expect("read-only property has read type");
                match relate_type_id_type_id(left_read_ty, right_read_ty) {
                    Relation::Coincident | Relation::Superset => return left,
                    Relation::Subset => return right,
                    Relation::Disjoint | Relation::Intersects => {
                        let prop_ty = self.union_(left_read_ty, right_read_ty);
                        let mut result = TableType::table_type_table_state_type_level_scope(
                            TableState::Sealed,
                            TypeLevel::default(),
                            core::ptr::null_mut(),
                        );
                        result
                            .props
                            .insert(prop_name.clone(), Property::readonly(prop_ty));
                        return unsafe { (*self.arena.cast_mut()).add_type(result) };
                    }
                }
            }
        }

        unsafe {
            (*self.arena.cast_mut()).add_type(UnionType {
                options: alloc::vec![left, right],
            })
        }
    }
}
