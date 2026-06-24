use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_table_type::get_mutable_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_string::is_string;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::functions::reduce_union::reduce_union;
use crate::records::any_type::AnyType;
use crate::records::error_type::ErrorType;
use crate::records::extern_type::ExternType;
use crate::records::intersection_type::IntersectionType;
use crate::records::missing_union_property::MissingUnionProperty;
use crate::records::never_type::NeverType;
use crate::records::property_type::Property;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::records::unknown_property::UnknownProperty;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn get_index_type_from_type_impl(
        &mut self,
        scope: ScopePtr,
        ty: TypeId,
        name: &Name,
        location: &Location,
        add_errors: bool,
    ) -> Option<TypeId> {
        let mut ty = unsafe { follow_type_id(ty) };

        if !unsafe { get_type_id::<ErrorType>(ty) }.is_null()
            || !unsafe { get_type_id::<AnyType>(ty) }.is_null()
            || !unsafe { get_type_id::<NeverType>(ty) }.is_null()
        {
            return Some(ty);
        }

        self.tablify(ty);

        if is_string(ty) {
            let mt_index = self.find_metatable_entry(
                self.string_type,
                alloc::string::String::from("__index"),
                location,
                add_errors,
            );
            LUAU_ASSERT!(mt_index.is_some());
            ty = mt_index.unwrap();
        }

        let table_type = get_mutable_table_type(ty);
        if !table_type.is_null() {
            let table_type = unsafe { &mut *table_type };
            if let Some(prop) = table_type.props.get(name) {
                return Some(prop.type_deprecated());
            } else if let Some(indexer) = table_type.indexer.clone() {
                // TODO: Property lookup should work with string singletons or unions thereof as the indexer key type.
                let errors = self.try_unify(self.string_type, indexer.index_type, &scope, location);

                if errors.is_empty() {
                    return Some(indexer.index_result_type);
                }

                if add_errors {
                    self.report_error_location_type_error_data(
                        location,
                        TypeErrorData::UnknownProperty(UnknownProperty {
                            table: ty,
                            key: name.clone(),
                        }),
                    );
                }

                return None;
            } else if table_type.state == TableState::Free {
                let result = self.fresh_type_type_level(table_type.level);
                let table_type = unsafe { &mut *get_mutable_table_type(ty) };
                table_type.props.insert(
                    name.clone(),
                    Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
                        result,
                        false,
                        alloc::string::String::new(),
                        None,
                        Default::default(),
                        None,
                        None,
                    ),
                );
                return Some(result);
            }

            if let Some(found) =
                self.find_table_property_respecting_meta(ty, name.clone(), location, add_errors)
            {
                return Some(found);
            }
        } else if !unsafe { get_type_id::<ExternType>(ty) }.is_null() {
            let cls = unsafe { &*get_type_id::<ExternType>(ty) };
            let prop = lookup_extern_type_prop(cls, name);
            if !prop.is_null() {
                return Some(unsafe { &*prop }.type_deprecated());
            }

            if let Some(indexer) = cls.indexer.clone() {
                // TODO: Property lookup should work with string singletons or unions thereof as the indexer key type.
                let errors = self.try_unify(self.string_type, indexer.index_type, &scope, location);

                if errors.is_empty() {
                    return Some(indexer.index_result_type);
                }

                if add_errors {
                    self.report_error_location_type_error_data(
                        location,
                        TypeErrorData::UnknownProperty(UnknownProperty {
                            table: ty,
                            key: name.clone(),
                        }),
                    );
                }

                return None;
            }
        } else if !unsafe { get_type_id::<UnionType>(ty) }.is_null() {
            let utv = unsafe { &*get_type_id::<UnionType>(ty) };
            let mut good_options: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
            let mut bad_options: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();

            for &t in utv.options.iter() {
                let mut _rl = RecursionLimiter {
                    base: unsafe { core::mem::zeroed() },
                    native_stack_guard: unsafe { core::mem::zeroed() },
                };
                _rl.recursion_limiter_recursion_limiter(
                    "TypeInfer::UnionType",
                    &mut self.recursion_count,
                    luaur_common::FInt::LuauTypeInferRecursionLimit.get() as core::ffi::c_int,
                );

                // Not needed when we normalize typeArguments.
                if !unsafe { get_type_id::<AnyType>(follow_type_id(t)) }.is_null() {
                    return Some(t);
                }

                if let Some(ty2) =
                    self.get_index_type_from_type(scope.clone(), t, name, location, false)
                {
                    good_options.push(ty2);
                } else {
                    bad_options.push(t);
                }
            }

            if !bad_options.is_empty() {
                if add_errors {
                    if good_options.is_empty() {
                        self.report_error_location_type_error_data(
                            location,
                            TypeErrorData::UnknownProperty(UnknownProperty {
                                table: ty,
                                key: name.clone(),
                            }),
                        );
                    } else {
                        self.report_error_location_type_error_data(
                            location,
                            TypeErrorData::MissingUnionProperty(MissingUnionProperty {
                                r#type: ty,
                                missing: bad_options,
                                key: name.clone(),
                            }),
                        );
                    }
                }
                return None;
            }

            let result = reduce_union(&good_options);
            if result.is_empty() {
                return Some(self.never_type);
            }

            if result.len() == 1 {
                return Some(result[0]);
            }

            return Some(unsafe {
                let module = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                    as *mut crate::records::module::Module;
                (*module)
                    .internal_types
                    .add_type(UnionType { options: result })
            });
        } else if !unsafe { get_type_id::<IntersectionType>(ty) }.is_null() {
            let itv = unsafe { &*get_type_id::<IntersectionType>(ty) };
            let mut parts: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();

            for &t in itv.parts.iter() {
                let mut _rl = RecursionLimiter {
                    base: unsafe { core::mem::zeroed() },
                    native_stack_guard: unsafe { core::mem::zeroed() },
                };
                _rl.recursion_limiter_recursion_limiter(
                    "TypeInfer::IntersectionType",
                    &mut self.recursion_count,
                    luaur_common::FInt::LuauTypeInferRecursionLimit.get() as core::ffi::c_int,
                );

                if let Some(ty2) =
                    self.get_index_type_from_type(scope.clone(), t, name, location, false)
                {
                    parts.push(ty2);
                }
            }

            // If no parts of the intersection had the property we looked up for, it never existed at all.
            if parts.is_empty() {
                if add_errors {
                    self.report_error_location_type_error_data(
                        location,
                        TypeErrorData::UnknownProperty(UnknownProperty {
                            table: ty,
                            key: name.clone(),
                        }),
                    );
                }
                return None;
            }

            if parts.len() == 1 {
                return Some(parts[0]);
            }

            return Some(unsafe {
                let module = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                    as *mut crate::records::module::Module;
                (*module)
                    .internal_types
                    .add_type(IntersectionType { parts })
            }); // Not at all correct.
        }

        if add_errors {
            self.report_error_location_type_error_data(
                location,
                TypeErrorData::UnknownProperty(UnknownProperty {
                    table: ty,
                    key: name.clone(),
                }),
            );
        }

        None
    }
}
