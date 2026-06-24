use crate::enums::relation::Relation;
use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_approximately_falsy_type::is_approximately_falsy_type;
use crate::functions::is_approximately_truthy_type::is_approximately_truthy_type;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::error_type::ErrorType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::variant::Variant2;

impl TypeSimplifier {
    pub fn basic_intersect(&mut self, left: TypeId, right: TypeId) -> Option<TypeId> {
        let builtin_types = unsafe { &*self.builtin_types };
        let left = unsafe { follow_type_id(left) };
        let right = unsafe { follow_type_id(right) };

        if !unsafe { get_type_id::<AnyType>(left) }.is_null()
            && !unsafe { get_type_id::<ErrorType>(right) }.is_null()
        {
            return Some(right);
        }
        if !unsafe { get_type_id::<AnyType>(right) }.is_null()
            && !unsafe { get_type_id::<ErrorType>(left) }.is_null()
        {
            return Some(left);
        }
        if !unsafe { get_type_id::<AnyType>(left) }.is_null() {
            let arena = unsafe { &mut *self.arena.cast_mut() };
            return Some(arena.add_type(UnionType {
                options: alloc::vec![right, builtin_types.errorType],
            }));
        }
        if !unsafe { get_type_id::<AnyType>(right) }.is_null() {
            let arena = unsafe { &mut *self.arena.cast_mut() };
            return Some(arena.add_type(UnionType {
                options: alloc::vec![left, builtin_types.errorType],
            }));
        }
        if !unsafe { get_type_id::<UnknownType>(left) }.is_null() {
            return Some(right);
        }
        if !unsafe { get_type_id::<UnknownType>(right) }.is_null() {
            return Some(left);
        }
        if !unsafe { get_type_id::<NeverType>(left) }.is_null() {
            return Some(left);
        }
        if !unsafe { get_type_id::<NeverType>(right) }.is_null() {
            return Some(right);
        }

        if let Some(pt) = unsafe { get_type_id::<PrimitiveType>(left).as_ref() } {
            if pt.r#type == PrimitiveType::Boolean {
                if let Some(st) = unsafe { get_type_id::<SingletonType>(right).as_ref() } {
                    if st.variant.get_if_0().is_some() {
                        return Some(right);
                    }
                }
                if let Some(nt) = unsafe { get_type_id::<NegationType>(right).as_ref() } {
                    if let Some(st) =
                        unsafe { get_type_id::<SingletonType>(follow_type_id(nt.ty)).as_ref() }
                    {
                        if st.variant.get_if_0().is_some() {
                            if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
                                return Some(builtin_types.falseType);
                            } else {
                                return Some(builtin_types.trueType);
                            }
                        }
                    }
                }
            }
        } else if let Some(pt) = unsafe { get_type_id::<PrimitiveType>(right).as_ref() } {
            if pt.r#type == PrimitiveType::Boolean {
                if let Some(st) = unsafe { get_type_id::<SingletonType>(left).as_ref() } {
                    if st.variant.get_if_0().is_some() {
                        return Some(left);
                    }
                }
                if let Some(nt) = unsafe { get_type_id::<NegationType>(left).as_ref() } {
                    if let Some(st) =
                        unsafe { get_type_id::<SingletonType>(follow_type_id(nt.ty)).as_ref() }
                    {
                        if st.variant.get_if_0().is_some() {
                            if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
                                return Some(builtin_types.falseType);
                            } else {
                                return Some(builtin_types.trueType);
                            }
                        }
                    }
                }
            }
        }

        if let (Some(lt), Some(rt)) = unsafe {
            (
                get_type_id::<TableType>(left).as_ref(),
                get_type_id::<TableType>(right).as_ref(),
            )
        } {
            if lt.props.len() == 1 {
                let (prop_name, left_prop) = lt.props.iter().next().unwrap();
                let left_prop_is_refinable = left_prop.is_shared() || left_prop.is_read_only();

                if let Some(right_prop) = rt.props.get(prop_name) {
                    if left_prop_is_refinable && right_prop.is_shared() {
                        let relation = relate_type_id_type_id(
                            left_prop.read_ty.expect("refinable property has read type"),
                            right_prop.read_ty.expect("shared property has read type"),
                        );

                        match relation {
                            Relation::Disjoint => return Some(builtin_types.neverType),
                            Relation::Superset | Relation::Coincident => return Some(right),
                            Relation::Subset => {
                                if rt.props.len() == 1 && left_prop.is_shared() {
                                    return Some(left);
                                }
                            }
                            Relation::Intersects => {}
                        }
                    }
                }
            } else if rt.props.len() == 1 {
                return self.basic_intersect(right, left);
            }

            if lt.indexer.is_none()
                && rt.indexer.is_none()
                && lt.state == TableState::Sealed
                && rt.state == TableState::Sealed
            {
                if rt.props.is_empty() {
                    return Some(left);
                }

                let are_disjoint = lt.props.keys().all(|name| !rt.props.contains_key(name));

                if are_disjoint {
                    let mut merged = TableType::table_type_table_state_type_level_scope(
                        TableState::Sealed,
                        TypeLevel::default(),
                        lt.scope,
                    );
                    merged.props = lt.props.clone();

                    for (name, prop) in &rt.props {
                        merged.props.insert(name.clone(), prop.clone());
                    }

                    let arena = unsafe { &mut *self.arena.cast_mut() };
                    return Some(arena.add_type(merged));
                }
            }

            return None;
        }

        if is_approximately_truthy_type(left) {
            if let Some(res) = self.basic_intersect_with_truthy(right) {
                return Some(res);
            }
        }

        if is_approximately_truthy_type(right) {
            if let Some(res) = self.basic_intersect_with_truthy(left) {
                return Some(res);
            }
        }

        if is_approximately_falsy_type(left) {
            if let Some(res) = self.basic_intersect_with_falsy(right) {
                return Some(res);
            }
        }

        if is_approximately_falsy_type(right) {
            if let Some(res) = self.basic_intersect_with_falsy(left) {
                return Some(res);
            }
        }

        let relation = relate_type_id_type_id(left, right);
        if left == right || Relation::Coincident == relation {
            return Some(left);
        }
        match relation {
            Relation::Disjoint => Some(builtin_types.neverType),
            Relation::Subset => Some(left),
            Relation::Superset => Some(right),
            _ => None,
        }
    }
}
