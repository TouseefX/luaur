use crate::enums::table_state::TableState;
use crate::enums::value_context::ValueContext;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_table_type::get_mutable_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_table_intersection::is_table_intersection;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::functions::reduce_union::reduce_union;
use crate::records::any_type::AnyType;
use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe;
use crate::records::extern_type::ExternType;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::not_a_table::NotATable;
use crate::records::property_type::Property;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::union_type::UnionType;
use crate::records::unknown_property::UnknownProperty;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeChecker {
    pub fn check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprIndexExpr,
        ctx: ValueContext,
    ) -> TypeId {
        let expr_type = self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.expr },
                None,
                false,
            )
            .r#type;
        self.tablify(expr_type);
        let expr_type =
            self.strip_from_nil_and_report(expr_type, unsafe { &(*expr.expr).base.location });
        let index_type = self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.index },
                None,
                false,
            )
            .r#type;
        let expr_type = unsafe { follow_type_id(expr_type) };

        if unsafe { !get_type_id::<AnyType>(expr_type).is_null() }
            || unsafe { !get_type_id::<ErrorType>(expr_type).is_null() }
        {
            return expr_type;
        }

        if unsafe { !get_type_id::<NeverType>(expr_type).is_null() } {
            return self.unknown_type;
        }

        let value = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantString>(expr.index as *mut AstNode)
        };

        if !value.is_null() {
            let value_name: Name = unsafe {
                let slice = core::slice::from_raw_parts(
                    (*value).value.data as *const u8,
                    (*value).value.size,
                );
                core::str::from_utf8_unchecked(slice).to_string()
            };

            if unsafe { !get_type_id::<ExternType>(expr_type).is_null() } {
                let expr_extern_type = unsafe { &*get_type_id::<ExternType>(expr_type) };
                let prop = lookup_extern_type_prop(expr_extern_type, &value_name);
                if !prop.is_null() {
                    let prop = unsafe { &*prop };
                    if ctx == ValueContext::LValue {
                        if let Some(write_ty) = prop.write_ty {
                            return write_ty;
                        }
                    }

                    return prop.type_deprecated();
                }

                if let Some(ref indexer) = expr_extern_type.indexer {
                    self.unify_type_id_type_id_scope_ptr_location(
                        self.string_type,
                        indexer.index_type,
                        scope,
                        unsafe { &(*expr.index).base.location },
                    );
                    return indexer.index_result_type;
                }

                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    expr.base.base.location,
                    TypeErrorData::UnknownProperty(UnknownProperty {
                        table: expr_type,
                        key: value_name,
                    }),
                ));
                return self.error_recovery_type_scope_ptr(scope);
            } else if unsafe { !get_type_id::<IntersectionType>(expr_type).is_null() } {
                let name = value_name;

                if let Some(ty) = self.get_index_type_from_type(
                    scope.clone(),
                    expr_type,
                    &name,
                    &expr.base.base.location,
                    false,
                ) {
                    return ty;
                }

                if is_table_intersection(expr_type) {
                    self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                        expr.base.base.location,
                        TypeErrorData::CannotExtendTable(CannotExtendTable {
                            table_type: expr_type,
                            context: crate::records::cannot_extend_table::Context::Property,
                            prop: name,
                        }),
                    ));
                    return self.error_recovery_type_scope_ptr(scope);
                }
            }
        } else {
            if unsafe { !get_type_id::<ExternType>(expr_type).is_null() } {
                let expr_extern_type = unsafe { &*get_type_id::<ExternType>(expr_type) };
                if let Some(ref indexer) = expr_extern_type.indexer {
                    self.unify_type_id_type_id_scope_ptr_location(
                        index_type,
                        indexer.index_type,
                        scope,
                        unsafe { &(*expr.index).base.location },
                    );
                    return indexer.index_result_type;
                }
            }

            if unsafe { !get_type_id::<ExternType>(expr_type).is_null() } {
                if self.is_nonstrict_mode() {
                    return self.unknown_type;
                }
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    expr.base.base.location,
                    TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(
                        DynamicPropertyLookupOnExternTypesUnsafe { ty: expr_type },
                    ),
                ));
                return self.error_recovery_type_scope_ptr(scope);
            }
        }

        let mut table_types: Vec<*mut TableType> = Vec::new();
        let mut is_union = true;

        if unsafe { !get_type_id::<UnionType>(expr_type).is_null() } {
            let expr_union = unsafe { &*get_type_id::<UnionType>(expr_type) };
            table_types.reserve(expr_union.options.len());

            for option in &expr_union.options {
                let option_table = get_mutable_table_type(*option);
                if option_table.is_null() {
                    self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                        unsafe { (*expr.expr).base.location },
                        TypeErrorData::NotATable(NotATable { ty: expr_type }),
                    ));
                    return self.error_recovery_type_scope_ptr(scope);
                }
                table_types.push(option_table);
            }
        } else if unsafe { !get_type_id::<IntersectionType>(expr_type).is_null() } {
            let expr_intersection = unsafe { &*get_type_id::<IntersectionType>(expr_type) };
            table_types.reserve(expr_intersection.parts.len());
            is_union = false;

            for part in &expr_intersection.parts {
                let part_table = get_mutable_table_type(*part);
                if part_table.is_null() {
                    self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                        unsafe { (*expr.expr).base.location },
                        TypeErrorData::NotATable(NotATable { ty: expr_type }),
                    ));
                    return self.error_recovery_type_scope_ptr(scope);
                }
                table_types.push(part_table);
            }
        } else {
            let expr_table = get_mutable_table_type(expr_type);
            if !expr_table.is_null() {
                table_types.push(expr_table);
            } else {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    unsafe { (*expr.expr).base.location },
                    TypeErrorData::NotATable(NotATable { ty: expr_type }),
                ));
                return self.error_recovery_type_scope_ptr(scope);
            }
        }

        if !value.is_null() {
            let key_name: Name = unsafe {
                let slice = core::slice::from_raw_parts(
                    (*value).value.data as *const u8,
                    (*value).value.size,
                );
                core::str::from_utf8_unchecked(slice).to_string()
            };
            let mut property_types: DenseHashSet<TypeId> = DenseHashSet::new(TypeId::default());

            for table in &table_types {
                if let Some(prop) = unsafe { (**table).props.get(&key_name) } {
                    property_types.insert(prop.type_deprecated());
                } else if (ctx == ValueContext::LValue
                    && unsafe { (**table).state } == TableState::Unsealed)
                    || unsafe { (**table).state } == TableState::Free
                {
                    let result_type = self.fresh_type_scope_ptr(scope.clone());
                    let property = unsafe {
                        (**table)
                            .props
                            .entry(key_name.clone())
                            .or_insert_with(Property::property)
                    };
                    property.set_type(result_type);
                    property.location = Some(unsafe { (*expr.index).base.location });
                    property_types.insert(result_type);
                }
            }

            if property_types.size() == 1 {
                if let Some(ty) = property_types.iter().next() {
                    return *ty;
                }
            }

            if !property_types.empty() {
                if is_union {
                    let mut options_vec = Vec::new();
                    for ty in property_types.iter() {
                        options_vec.push(*ty);
                    }
                    let options = reduce_union(&options_vec);
                    if options.is_empty() {
                        return self.never_type;
                    }
                    if options.len() == 1 {
                        return options[0];
                    }
                    return self.add_type(&UnionType { options });
                }

                let mut parts_vec = Vec::new();
                for ty in property_types.iter() {
                    parts_vec.push(*ty);
                }
                return self.add_type(&IntersectionType { parts: parts_vec });
            }
        }

        let mut result_types: DenseHashSet<TypeId> = DenseHashSet::new(TypeId::default());

        for table in &table_types {
            let table_ref = unsafe { &**table };
            if let Some(ref indexer) = table_ref.indexer {
                self.unify_type_id_type_id_scope_ptr_location(
                    index_type,
                    indexer.index_type,
                    scope,
                    unsafe { &(*expr.index).base.location },
                );
                result_types.insert(indexer.index_result_type);
            } else if (ctx == ValueContext::LValue && table_ref.state == TableState::Unsealed)
                || table_ref.state == TableState::Free
            {
                let level = table_ref.level;
                let indexer_type = self.fresh_type_type_level(level);
                self.unify_type_id_type_id_scope_ptr_location(
                    index_type,
                    indexer_type,
                    scope,
                    &expr.base.base.location,
                );
                let index_result_type = self.fresh_type_type_level(level);

                let index_type_any = self.any_if_nonstrict(indexer_type);
                let index_result_type_any = self.any_if_nonstrict(index_result_type);
                unsafe {
                    (**table).indexer = Some(TableIndexer {
                        index_type: index_type_any,
                        index_result_type: index_result_type_any,
                        is_read_only: false,
                    });
                }
                result_types.insert(index_result_type);
            } else {
                if is_union {
                    return self.any_type;
                }
                result_types.insert(self.any_type);
            }
        }

        if result_types.size() == 1 {
            if let Some(ty) = result_types.iter().next() {
                return *ty;
            }
        }

        if is_union {
            let mut options_vec = Vec::new();
            for ty in result_types.iter() {
                options_vec.push(*ty);
            }
            let options = reduce_union(&options_vec);
            if options.is_empty() {
                return self.never_type;
            }
            if options.len() == 1 {
                return options[0];
            }
            return self.add_type(&UnionType { options });
        }

        let mut parts_vec = Vec::new();
        for ty in result_types.iter() {
            parts_vec.push(*ty);
        }
        self.add_type(&IntersectionType { parts: parts_vec })
    }
}
