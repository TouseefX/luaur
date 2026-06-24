use crate::enums::table_state::TableState;
use crate::enums::value_context::ValueContext;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_table_type::get_mutable_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_table_intersection::is_table_intersection;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::records::any_type::AnyType;
use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::extern_type::ExternType;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::not_a_table::NotATable;
use crate::records::property_type::Property;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::unknown_property::UnknownProperty;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl TypeChecker {
    pub fn check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprIndexName,
        ctx: ValueContext,
    ) -> TypeId {
        let mut lhs = self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.expr },
                None,
                false,
            )
            .r#type;

        if unsafe { !get_type_id::<ErrorType>(lhs).is_null() }
            || unsafe { !get_type_id::<AnyType>(lhs).is_null() }
        {
            return lhs;
        }

        if unsafe { !get_type_id::<NeverType>(lhs).is_null() } {
            return self.unknown_type;
        }

        self.tablify(lhs);

        let name: Name = unsafe {
            core::ffi::CStr::from_ptr(expr.index.value)
                .to_string_lossy()
                .into_owned()
        };

        lhs = self.strip_from_nil_and_report(lhs, unsafe { &(*expr.expr).base.location });

        let lhs_table = get_mutable_table_type(lhs);
        if !lhs_table.is_null() {
            let lhs_table_ref = unsafe { &mut *lhs_table };
            if let Some(prop) = lhs_table_ref.props.get(&name) {
                return prop.type_deprecated();
            } else if (ctx == ValueContext::LValue && lhs_table_ref.state == TableState::Unsealed)
                || lhs_table_ref.state == TableState::Free
            {
                let the_type = self.fresh_type_scope_ptr(scope.clone());
                let property = lhs_table_ref
                    .props
                    .entry(name)
                    .or_insert_with(Property::default);
                property.set_type(the_type);
                property.location = Some(expr.index_location);
                return the_type;
            } else if let Some(indexer) = lhs_table_ref.indexer.clone() {
                let ok = self.unify_type_id_type_id_scope_ptr_location(
                    self.string_type,
                    indexer.index_type,
                    scope,
                    &expr.base.base.location,
                );
                let mut ret_type = indexer.index_result_type;
                if !ok {
                    self.report_error_location_type_error_data(
                        &expr.base.base.location,
                        TypeErrorData::UnknownProperty(UnknownProperty {
                            table: lhs,
                            key: name,
                        }),
                    );
                    ret_type = self.error_recovery_type_type_id(ret_type);
                }
                return ret_type;
            } else if lhs_table_ref.state == TableState::Sealed {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    expr.base.base.location,
                    TypeErrorData::CannotExtendTable(CannotExtendTable {
                        table_type: lhs,
                        context: crate::records::cannot_extend_table::Context::Property,
                        prop: name,
                    }),
                ));
                return self.error_recovery_type_scope_ptr(scope);
            } else {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    expr.base.base.location,
                    TypeErrorData::GenericError(crate::records::generic_error::GenericError::new(
                        alloc::string::String::from(
                            "Internal error: generic tables are not lvalues",
                        ),
                    )),
                ));
                return self.error_recovery_type_scope_ptr(scope);
            }
        } else if unsafe { !get_type_id::<ExternType>(lhs).is_null() } {
            let lhs_extern_type = unsafe { &*get_type_id::<ExternType>(lhs) };
            let prop = lookup_extern_type_prop(lhs_extern_type, &name);
            if !prop.is_null() {
                let prop = unsafe { &*prop };
                if ctx == ValueContext::LValue {
                    if let Some(write_ty) = prop.write_ty {
                        return write_ty;
                    }
                }

                return prop.type_deprecated();
            }

            if let Some(indexer) = lhs_extern_type.indexer.clone() {
                let ok = self.unify_type_id_type_id_scope_ptr_location(
                    self.string_type,
                    indexer.index_type,
                    scope,
                    &expr.base.base.location,
                );
                if ok {
                    return indexer.index_result_type;
                }
            }

            self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                expr.base.base.location,
                TypeErrorData::UnknownProperty(UnknownProperty {
                    table: lhs,
                    key: name,
                }),
            ));
            return self.error_recovery_type_scope_ptr(scope);
        } else if unsafe { !get_type_id::<IntersectionType>(lhs).is_null() } {
            if let Some(ty) = self.get_index_type_from_type(
                scope.clone(),
                lhs,
                &name,
                &expr.base.base.location,
                false,
            ) {
                return ty;
            }

            // If intersection has a table part, report that it cannot be extended just as a sealed table
            if is_table_intersection(lhs) {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    expr.base.base.location,
                    TypeErrorData::CannotExtendTable(CannotExtendTable {
                        table_type: lhs,
                        context: crate::records::cannot_extend_table::Context::Property,
                        prop: name,
                    }),
                ));
                return self.error_recovery_type_scope_ptr(scope);
            }
        }

        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            TypeErrorData::NotATable(NotATable { ty: lhs }),
        ));
        self.error_recovery_type_scope_ptr(scope)
    }
}
