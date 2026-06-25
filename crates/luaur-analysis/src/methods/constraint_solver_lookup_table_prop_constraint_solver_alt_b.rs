use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;
use crate::enums::value_context::ValueContext;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::fast_is_subtype::fast_is_subtype;
use crate::functions::follow_type::follow_type_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::any_type::AnyType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::property_type::Property;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_prop_lookup_result::TablePropLookupResult;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::records::union_type::UnionType;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ConstraintSolver {
    pub fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
        &mut self,
        constraint: *const Constraint,
        subject_type: TypeId,
        prop_name: &str,
        context: ValueContext,
        in_conditional: bool,
        suppress_simplification: bool,
        seen: &mut DenseHashSet<TypeId>,
    ) -> TablePropLookupResult {
        if seen.contains(&subject_type) {
            return TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: None,
                is_index: false,
            };
        }

        let mut seen = seen.clone();
        seen.insert(subject_type);

        let subject_type = unsafe { follow_type_id(subject_type) };

        if self.is_blocked_type_id(subject_type) {
            return TablePropLookupResult {
                blocked_types: vec![subject_type],
                prop_type: None,
                is_index: false,
            };
        } else if !unsafe { get_type_id::<AnyType>(subject_type) }.is_null()
            || !unsafe { get_type_id::<NeverType>(subject_type) }.is_null()
        {
            return TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: Some(subject_type),
                is_index: false,
            };
        } else if !unsafe { getMutable::<TableType>(subject_type) }.is_null() {
            let ttv = unsafe { &mut *getMutable::<TableType>(subject_type) };

            if let Some(prop) = ttv.props.get(prop_name) {
                match context {
                    ValueContext::RValue => {
                        if let Some(read_ty) = prop.read_ty {
                            return TablePropLookupResult {
                                blocked_types: Vec::new(),
                                prop_type: Some(read_ty),
                                is_index: false,
                            };
                        }
                    }
                    ValueContext::LValue => {
                        if let Some(write_ty) = prop.write_ty {
                            return TablePropLookupResult {
                                blocked_types: Vec::new(),
                                prop_type: Some(write_ty),
                                is_index: false,
                            };
                        }
                    }
                }
            }

            if let Some(indexer) = ttv.indexer {
                if self.is_blocked_type_id(indexer.index_type) {
                    return TablePropLookupResult {
                        blocked_types: vec![indexer.index_type],
                        prop_type: None,
                        is_index: true,
                    };
                }

                // CLI-169235: build a faux string-singleton literal from the prop
                // name and reuse subtyping (same logic as `index<_, _>`), so a named
                // access hits the indexer when the name is a subtype of the index
                // key (e.g. "Val1" against a `"Val1"|"Val2"|"Val3"` key), not just
                // when the key is a plain string.
                let faux_literal = unsafe {
                    (*self.arena).add_type(SingletonType::singleton_type(SingletonVariant::V1(
                        StringSingleton::new(prop_name.to_string()),
                    )))
                };
                if fast_is_subtype(faux_literal, indexer.index_type) {
                    return TablePropLookupResult {
                        blocked_types: Vec::new(),
                        prop_type: Some(indexer.index_result_type),
                        is_index: true,
                    };
                }
            }

            if ttv.state == TableState::Free {
                let result = unsafe {
                    fresh_type(
                        &mut *self.arena,
                        &*self.builtin_types,
                        ttv.scope,
                        Polarity::Mixed,
                    )
                };
                track_interior_free_type(ttv.scope, result);

                match context {
                    ValueContext::RValue => {
                        ttv.props
                            .insert(prop_name.to_string(), Property::readonly(result));
                    }
                    ValueContext::LValue => {
                        if let Some(prop) = ttv.props.get_mut(prop_name) {
                            if prop.is_read_only() {
                                prop.write_ty = prop.read_ty;
                                return TablePropLookupResult {
                                    blocked_types: Vec::new(),
                                    prop_type: prop.read_ty,
                                    is_index: false,
                                };
                            }
                        }

                        ttv.props
                            .insert(prop_name.to_string(), Property::rw_type_id(result));
                    }
                }

                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: Some(result),
                    is_index: false,
                };
            }

            if in_conditional {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: Some(unsafe { (*self.builtin_types).unknownType }),
                    is_index: false,
                };
            }

            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: None,
                is_index: false,
            }
        } else if let Some(mt) = unsafe { get_type_id::<MetatableType>(subject_type).as_ref() } {
            if context == ValueContext::LValue {
                return self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                    constraint,
                    mt.table,
                    prop_name,
                    context,
                    in_conditional,
                    suppress_simplification,
                    &mut seen,
                );
            }

            let result = self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                constraint,
                mt.table,
                prop_name,
                context,
                in_conditional,
                suppress_simplification,
                &mut seen,
            );
            if !result.blocked_types.is_empty() || result.prop_type.is_some() {
                return result;
            }

            let metatable = unsafe { follow_type_id(mt.metatable) };
            if self.is_blocked_type_id(metatable) {
                return TablePropLookupResult {
                    blocked_types: vec![metatable],
                    prop_type: None,
                    is_index: false,
                };
            }

            if let Some(mtt) = get_table_type(metatable) {
                let Some(index_prop) = mtt.props.get("__index") else {
                    return result;
                };

                if index_prop.is_write_only() {
                    return TablePropLookupResult {
                        blocked_types: Vec::new(),
                        prop_type: Some(unsafe { (*self.builtin_types).errorType }),
                        is_index: false,
                    };
                }

                if let Some(index_type) = index_prop.read_ty {
                    let index_type = unsafe { follow_type_id(index_type) };
                    if let Some(ft) = unsafe { get_type_id::<FunctionType>(index_type).as_ref() } {
                        let rets = unsafe {
                            extend_type_pack(
                                &mut *self.arena,
                                self.builtin_types,
                                ft.ret_types,
                                1,
                                Vec::new(),
                            )
                        };
                        return TablePropLookupResult {
                            blocked_types: Vec::new(),
                            prop_type: if rets.head.len() == 1 {
                                Some(rets.head[0])
                            } else {
                                Some(unsafe { (*self.builtin_types).nilType })
                            },
                            is_index: false,
                        };
                    }

                    return self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                        constraint,
                        index_type,
                        prop_name,
                        context,
                        in_conditional,
                        suppress_simplification,
                        &mut seen,
                    );
                }

                result
            } else if unsafe { !get_type_id::<MetatableType>(metatable).is_null() } {
                self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                    constraint,
                    metatable,
                    prop_name,
                    context,
                    in_conditional,
                    suppress_simplification,
                    &mut seen,
                )
            } else {
                result
            }
        } else if let Some(cls) = unsafe { get_type_id::<ExternType>(subject_type).as_ref() } {
            let prop_name_string = prop_name.to_string();
            let prop = lookup_extern_type_prop(cls, &prop_name_string);
            if !prop.is_null() {
                let prop = unsafe { &*prop };
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: if context == ValueContext::RValue {
                        prop.read_ty
                    } else {
                        prop.write_ty
                    },
                    is_index: false,
                };
            }

            if let Some(indexer) = &cls.indexer {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: Some(indexer.index_result_type),
                    is_index: true,
                };
            }

            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: None,
                is_index: false,
            }
        } else if let Some(ft) = unsafe { get_type_id::<FreeType>(subject_type).as_ref() } {
            let upper_bound = unsafe { follow_type_id(ft.upper_bound) };

            if unsafe { !get_type_id::<TableType>(upper_bound).is_null() }
                || unsafe { !get_type_id::<PrimitiveType>(upper_bound).is_null() }
            {
                let res = self
                    .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                        constraint,
                        upper_bound,
                        prop_name,
                        context,
                        in_conditional,
                        suppress_simplification,
                        &mut seen,
                    );

                if res.prop_type.is_some() {
                    return res;
                }
            }

            let scope = ft.scope;
            let new_upper_bound = unsafe {
                (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
                    TableState::Free,
                    TypeLevel::default(),
                    scope,
                ))
            };

            track_interior_free_type(unsafe { (*constraint).scope }, new_upper_bound);

            let tt = unsafe { getMutable::<TableType>(new_upper_bound) };
            LUAU_ASSERT!(!tt.is_null());

            let prop_type = unsafe {
                fresh_type(
                    &mut *self.arena,
                    &*self.builtin_types,
                    scope,
                    Polarity::Mixed,
                )
            };
            track_interior_free_type(scope, prop_type);

            match context {
                ValueContext::RValue => unsafe {
                    (*tt)
                        .props
                        .insert(prop_name.to_string(), Property::readonly(prop_type));
                },
                ValueContext::LValue => unsafe {
                    (*tt)
                        .props
                        .insert(prop_name.to_string(), Property::rw_type_id(prop_type));
                },
            }

            self.constraint_solver_unify(constraint, subject_type, new_upper_bound);

            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: Some(prop_type),
                is_index: false,
            }
        } else if let Some(utv) = unsafe { get_type_id::<UnionType>(subject_type).as_ref() } {
            let mut blocked = Vec::new();
            let mut options = Vec::new();

            for ty in &utv.options {
                let result = self
                    .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                        constraint,
                        *ty,
                        prop_name,
                        context,
                        in_conditional,
                        suppress_simplification,
                        &mut seen,
                    );

                for blocked_ty in result.blocked_types {
                    if !blocked.contains(&blocked_ty) {
                        blocked.push(blocked_ty);
                    }
                }

                if let Some(prop_type) = result.prop_type {
                    if !options.contains(&prop_type) {
                        options.push(prop_type);
                    }
                }
            }

            if !blocked.is_empty() {
                return TablePropLookupResult {
                    blocked_types: blocked,
                    prop_type: None,
                    is_index: false,
                };
            }

            if options.is_empty() {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: None,
                    is_index: false,
                };
            }

            if options.len() == 1 {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: Some(options[0]),
                    is_index: false,
                };
            }

            let prop_type = if options.len() == 2 && !suppress_simplification {
                let scope = unsafe { (*constraint).scope };
                let location = unsafe { (*constraint).location };
                if context == ValueContext::LValue {
                    self.simplify_intersection_not_null_scope_location_type_id_type_id(
                        scope, location, options[0], options[1],
                    )
                } else {
                    self.simplify_union(scope, location, options[0], options[1])
                }
            } else if context == ValueContext::LValue {
                unsafe { (*self.arena).add_type(IntersectionType { parts: options }) }
            } else {
                unsafe { (*self.arena).add_type(UnionType { options }) }
            };

            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: Some(prop_type),
                is_index: false,
            }
        } else if let Some(itv) = unsafe { get_type_id::<IntersectionType>(subject_type).as_ref() }
        {
            let mut blocked = Vec::new();
            let mut options = Vec::new();

            for ty in &itv.parts {
                let result = self
                    .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                        constraint,
                        *ty,
                        prop_name,
                        context,
                        in_conditional,
                        suppress_simplification,
                        &mut seen,
                    );

                for blocked_ty in result.blocked_types {
                    if !blocked.contains(&blocked_ty) {
                        blocked.push(blocked_ty);
                    }
                }

                if let Some(prop_type) = result.prop_type {
                    if !options.contains(&prop_type) {
                        options.push(prop_type);
                    }
                }
            }

            if !blocked.is_empty() {
                return TablePropLookupResult {
                    blocked_types: blocked,
                    prop_type: None,
                    is_index: false,
                };
            }

            if options.is_empty() {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: None,
                    is_index: false,
                };
            }

            if options.len() == 1 {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: Some(options[0]),
                    is_index: false,
                };
            }

            let prop_type = if options.len() == 2 && !suppress_simplification {
                self.simplify_intersection_not_null_scope_location_type_id_type_id(
                    unsafe { (*constraint).scope },
                    unsafe { (*constraint).location },
                    options[0],
                    options[1],
                )
            } else {
                unsafe { (*self.arena).add_type(IntersectionType { parts: options }) }
            };

            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: Some(prop_type),
                is_index: false,
            }
        } else if let Some(pt) = unsafe { get_type_id::<PrimitiveType>(subject_type).as_ref() } {
            if pt.r#type == PrimitiveType::String && pt.metatable.is_some() {
                let metatable = pt.metatable.unwrap();
                let metatable = unsafe { follow_type_id(metatable) };
                let metatable_table = unsafe { get_type_id::<TableType>(metatable) };
                LUAU_ASSERT!(!metatable_table.is_null());

                let metatable_table = unsafe { &*metatable_table };
                if let Some(index_prop) = metatable_table.props.get("__index") {
                    if index_prop.is_write_only() {
                        return TablePropLookupResult {
                            blocked_types: Vec::new(),
                            prop_type: Some(unsafe { (*self.builtin_types).errorType }),
                            is_index: false,
                        };
                    }

                    if let Some(index_type) = index_prop.read_ty {
                        return self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                            constraint,
                            index_type,
                            prop_name,
                            context,
                            in_conditional,
                            suppress_simplification,
                            &mut seen,
                        );
                    }
                }
            }

            if in_conditional && pt.r#type == PrimitiveType::Table {
                return TablePropLookupResult {
                    blocked_types: Vec::new(),
                    prop_type: Some(unsafe { (*self.builtin_types).unknownType }),
                    is_index: false,
                };
            }

            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: None,
                is_index: false,
            }
        } else {
            TablePropLookupResult {
                blocked_types: Vec::new(),
                prop_type: None,
                is_index: false,
            }
        }
    }
}
