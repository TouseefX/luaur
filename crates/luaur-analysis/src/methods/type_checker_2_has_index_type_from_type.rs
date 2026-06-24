//! Faithful port of `TypeChecker2::hasIndexTypeFromType` (TypeChecker2.cpp:3727-3859).
use crate::enums::normalization_result::NormalizationResult;
use crate::enums::value_context::ValueContext;
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::find_table_property_respecting_meta_type_utils_alt_b::find_table_property_respecting_meta;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::in_conditional::in_conditional;
use crate::functions::is_string::is_string;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::records::any_type::AnyType;
use crate::records::extern_type::ExternType;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_error::TypeError;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

/// Faithful port of the local `struct PropertyType` from TypeChecker2.cpp:116-120
/// (`NormalizationResult present; std::optional<TypeId> result;`).
///
/// The shared `records::property_type::PropertyType` slot was repurposed as an
/// alias of `Property` (read/write types), so the result type of
/// `hasIndexTypeFromType` is defined locally to match the reference exactly.
#[derive(Debug, Clone)]
pub struct PropertyType {
    pub present: NormalizationResult,
    pub result: Option<TypeId>,
}

impl TypeChecker2 {
    pub fn has_index_type_from_type(
        &mut self,
        ty: TypeId,
        prop: &alloc::string::String,
        context: ValueContext,
        location: &Location,
        seen: &mut DenseHashSet<TypeId>,
        ast_index_expr_type: TypeId,
        errors: &mut alloc::vec::Vec<TypeError>,
    ) -> PropertyType {
        let mut ty = unsafe { follow_type_id(ty) };

        // If we have already encountered this type, we must assume that some
        // other codepath will do the right thing and signal false if the
        // property is not present.
        if seen.contains(&ty) {
            return PropertyType {
                present: NormalizationResult::True,
                result: None,
            };
        }
        seen.insert(ty);

        if !unsafe { get_type_id::<ErrorType>(ty) }.is_null()
            || !unsafe { get_type_id::<AnyType>(ty) }.is_null()
            || !unsafe { get_type_id::<NeverType>(ty) }.is_null()
        {
            return PropertyType {
                present: NormalizationResult::True,
                result: Some(ty),
            };
        }

        if is_string(ty) {
            let mt_index = find_metatable_entry(
                self.builtin_types,
                errors,
                unsafe { (*self.builtin_types).stringType },
                "__index",
                *location,
            );
            LUAU_ASSERT!(mt_index.is_some());
            ty = mt_index.unwrap();
        }

        if let Some(tt) = get_table_type(ty) {
            if let Some(res_ty) = find_table_property_respecting_meta(
                self.builtin_types,
                errors,
                ty,
                prop.as_str(),
                context,
                *location,
                /* useNewSolver */ true,
            ) {
                return PropertyType {
                    present: NormalizationResult::True,
                    result: Some(res_ty),
                };
            }

            if let Some(indexer) = &tt.indexer {
                let index_type = unsafe { follow_type_id(indexer.index_type) };
                let given_type = unsafe {
                    (*self.module)
                        .internal_types
                        .add_type(SingletonType::singleton_type(SingletonVariant::V1(
                            StringSingleton::new(prop.clone()),
                        )))
                };
                let module_scope = unsafe { (*self.module).get_module_scope() };
                let scope =
                    alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
                let key_matches = unsafe {
                    (*self.subtyping)
                        .is_subtype_type_id_type_id_not_null_scope(given_type, index_type, scope)
                        .is_subtype
                };

                if key_matches {
                    if luaur_common::FFlag::LuauReadOnlyIndexers.get()
                        && context == ValueContext::LValue
                        && indexer.is_read_only
                    {
                        return PropertyType {
                            present: NormalizationResult::False,
                            result: None,
                        };
                    }
                    return PropertyType {
                        present: NormalizationResult::True,
                        result: Some(indexer.index_result_type),
                    };
                }
            }

            PropertyType {
                present: NormalizationResult::False,
                result: Some(unsafe { (*self.builtin_types).unknownType }),
            }
        } else if !unsafe { get_type_id::<ExternType>(ty) }.is_null() {
            let cls = unsafe { &*get_type_id::<ExternType>(ty) };
            // If the property doesn't exist on the class, we consult the indexer.
            let property = lookup_extern_type_prop(cls, prop);
            if !property.is_null() {
                let property = unsafe { &*property };
                if (context == ValueContext::LValue && property.write_ty.is_none())
                    || (context == ValueContext::RValue && property.read_ty.is_none())
                {
                    return PropertyType {
                        present: NormalizationResult::False,
                        result: None,
                    };
                } else {
                    return PropertyType {
                        present: NormalizationResult::True,
                        result: if context == ValueContext::LValue {
                            property.write_ty
                        } else {
                            property.read_ty
                        },
                    };
                }
            }
            if let Some(indexer) = &cls.indexer {
                let inhabited_test_type = unsafe {
                    (*self.module).internal_types.add_type(IntersectionType {
                        parts: alloc::vec![indexer.index_type, ast_index_expr_type],
                    })
                };
                return PropertyType {
                    present: self.normalizer.is_inhabited_type_id(inhabited_test_type),
                    result: Some(indexer.index_result_type),
                };
            }

            if luaur_common::FFlag::DebugLuauUserDefinedClasses.get() {
                if let Some(metatable) = cls.metatable {
                    // For user-defined classes, the object metatable holds metamethods
                    // (e.g. __add) directly in its props rather than under an __index table.
                    let mtt = unsafe {
                        get_type_id::<crate::records::table_type::TableType>(follow_type_id(
                            metatable,
                        ))
                    };
                    if !mtt.is_null() {
                        let mtt = unsafe { &*mtt };
                        if let Some(mt_prop) = mtt.props.get(prop) {
                            if (context == ValueContext::LValue && mt_prop.write_ty.is_none())
                                || (context == ValueContext::RValue && mt_prop.read_ty.is_none())
                            {
                                return PropertyType {
                                    present: NormalizationResult::False,
                                    result: None,
                                };
                            }
                            return PropertyType {
                                present: NormalizationResult::True,
                                result: if context == ValueContext::LValue {
                                    mt_prop.write_ty
                                } else {
                                    mt_prop.read_ty
                                },
                            };
                        }
                    }
                }
            }

            PropertyType {
                present: NormalizationResult::False,
                result: None,
            }
        } else if !unsafe { get_type_id::<UnionType>(ty) }.is_null() {
            let utv = unsafe { &*get_type_id::<UnionType>(ty) };
            let mut parts: alloc::vec::Vec<TypeId> =
                alloc::vec::Vec::with_capacity(utv.options.len());

            for &part in utv.options.iter() {
                let result = self.has_index_type_from_type(
                    part,
                    prop,
                    context,
                    location,
                    seen,
                    ast_index_expr_type,
                    errors,
                );

                if result.present != NormalizationResult::True {
                    return PropertyType {
                        present: result.present,
                        result: None,
                    };
                }
                if let Some(r) = result.result {
                    parts.push(r);
                }
            }

            if parts.is_empty() {
                return PropertyType {
                    present: NormalizationResult::False,
                    result: None,
                };
            }

            if parts.len() == 1 {
                return PropertyType {
                    present: NormalizationResult::True,
                    result: Some(parts[0]),
                };
            }

            let prop_ty = if context == ValueContext::LValue {
                unsafe {
                    (*self.module)
                        .internal_types
                        .add_type(IntersectionType { parts })
                }
            } else {
                unsafe {
                    (*self.module)
                        .internal_types
                        .add_type(UnionType { options: parts })
                }
            };

            PropertyType {
                present: NormalizationResult::True,
                result: Some(prop_ty),
            }
        } else if !unsafe { get_type_id::<IntersectionType>(ty) }.is_null() {
            let itv = unsafe { &*get_type_id::<IntersectionType>(ty) };
            for &part in itv.parts.iter() {
                let result = self.has_index_type_from_type(
                    part,
                    prop,
                    context,
                    location,
                    seen,
                    ast_index_expr_type,
                    errors,
                );
                if result.present != NormalizationResult::False {
                    return result;
                }
            }

            PropertyType {
                present: NormalizationResult::False,
                result: None,
            }
        } else if !unsafe { get_type_id::<PrimitiveType>(ty) }.is_null() {
            let pt = unsafe { &*get_type_id::<PrimitiveType>(ty) };
            PropertyType {
                present: if in_conditional(self.type_context) && pt.r#type == PrimitiveType::Table {
                    NormalizationResult::True
                } else {
                    NormalizationResult::False
                },
                result: Some(ty),
            }
        } else {
            PropertyType {
                present: NormalizationResult::False,
                result: None,
            }
        }
    }
}
