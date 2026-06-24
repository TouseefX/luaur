use crate::enums::value::Value;
use crate::enums::value_context::ValueContext;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_optional::is_optional;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::not_a_table::NotATable;
use crate::records::optional_value_access::OptionalValueAccess;
use crate::records::property_access_violation::PropertyAccessViolation;
use crate::records::table_type::TableType;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_iterator::TypeIterator;
use crate::records::union_type::UnionType;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;

impl TypeChecker2 {
    pub fn visit_ast_expr_index_expr_value_context(
        &mut self,
        index_expr: *mut AstExprIndexExpr,
        context: ValueContext,
    ) {
        unsafe {
            let index = (*index_expr).index;
            let index_as_constant_string = luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
                index as *mut luaur_ast::records::ast_node::AstNode,
            );
            if !index_as_constant_string.is_null() {
                let ast_index_expr_type = self.lookup_type(index as *mut AstExpr);
                let sv = (*index_as_constant_string).value;
                let len = sv.size as usize;
                let data_ptr = sv.data;
                let string_value = core::slice::from_raw_parts(data_ptr as *const u8, len)
                    .iter()
                    .map(|&b| b as char)
                    .collect::<String>();

                self.visit_expr_name(
                    (*index_expr).expr,
                    (*index_expr).base.base.location,
                    &string_value,
                    context,
                    ast_index_expr_type,
                );
                return;
            }

            self.visit_ast_expr_value_context((*index_expr).expr, ValueContext::RValue);
            self.visit_ast_expr_value_context((*index_expr).index, ValueContext::RValue);

            let expr_type = follow_type_id(self.lookup_type((*index_expr).expr));
            let index_type = follow_type_id(self.lookup_type((*index_expr).index));

            if let Some(tt) = get_type_id::<TableType>(expr_type).as_ref() {
                if let Some(ref indexer) = (*tt).indexer {
                    self.test_is_subtype_type_id_type_id_location(
                        index_type,
                        indexer.index_type,
                        (*(*index_expr).index).base.location,
                    );
                    if crate::FFlag::LuauReadOnlyIndexers.get()
                        && context == ValueContext::LValue
                        && indexer.is_read_only
                    {
                        let err = PropertyAccessViolation {
                            table: expr_type,
                            key: "indexer".to_string(),
                            context:
                                crate::records::property_access_violation::Context::CannotWrite,
                        };
                        self.report_error_type_error_data_location(
                            crate::type_aliases::type_error_data::TypeErrorData::PropertyAccessViolation(err),
                            &(*index_expr).base.base.location,
                        );
                    }
                } else {
                    let err = CannotExtendTable {
                        table_type: expr_type,
                        context: crate::records::cannot_extend_table::Context::Indexer,
                        prop: "indexer??".to_string(),
                    };
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::CannotExtendTable(err),
                        &(*index_expr).base.base.location,
                    );
                }
            } else if let Some(mt) = get_type_id::<MetatableType>(expr_type).as_ref() {
                self.type_checker_2_index_expr_metatable_helper(
                    index_expr,
                    mt as *const MetatableType,
                    expr_type,
                    index_type,
                );
                return;
            } else if let Some(cls) =
                get_type_id::<crate::records::extern_type::ExternType>(expr_type).as_ref()
            {
                if (*cls).indexer.is_some() {
                    let indexer = (*cls).indexer.as_ref().unwrap();
                    self.test_is_subtype_type_id_type_id_location(
                        index_type,
                        indexer.index_type,
                        (*(*index_expr).index).base.location,
                    );
                } else {
                    let err = DynamicPropertyLookupOnExternTypesUnsafe { ty: expr_type };
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(err),
                        &(*index_expr).base.base.location,
                    );
                }
            } else if get_type_id::<UnionType>(expr_type).as_ref().is_some()
                && is_optional(expr_type)
            {
                let suppression = should_suppress_errors(&mut self.normalizer as *mut _, expr_type);
                match suppression.value {
                    Value::DoNotSuppress => {
                        let err = OptionalValueAccess {
                            optional: expr_type,
                        };
                        self.report_error_type_error_data_location(
                            crate::type_aliases::type_error_data::TypeErrorData::OptionalValueAccess(err),
                            &(*index_expr).base.base.location,
                        );
                    }
                    Value::NormalizationFailed => {
                        self.report_error_type_error_data_location(
                            crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(
                                NormalizationTooComplex::default(),
                            ),
                            &(*index_expr).base.base.location,
                        );
                        let err = OptionalValueAccess {
                            optional: expr_type,
                        };
                        self.report_error_type_error_data_location(
                            crate::type_aliases::type_error_data::TypeErrorData::OptionalValueAccess(err),
                            &(*index_expr).base.base.location,
                        );
                    }
                    Value::Suppress => {}
                }
            } else if let Some(ut) = get_type_id::<UnionType>(expr_type).as_ref() {
                // if all of the typeArguments are a table type, the union must be
                // a table, and so we shouldn't error.
                let mut all_tables = true;
                let mut union_iter = TypeIterator::<UnionType>::type_iterator_type(ut as *const _);
                let union_end = TypeIterator::<UnionType>::type_iterator_default();
                while union_iter.operator_ne(&union_end) {
                    let ty = union_iter.operator_deref();
                    union_iter.operator_inc();

                    if get_table_type(ty).is_none() {
                        all_tables = false;
                        break;
                    }
                }

                if !all_tables {
                    let suppression =
                        should_suppress_errors(&mut self.normalizer as *mut _, expr_type);
                    match suppression.value {
                        Value::DoNotSuppress => {
                            let err = NotATable { ty: expr_type };
                            self.report_error_type_error_data_location(
                                crate::type_aliases::type_error_data::TypeErrorData::NotATable(err),
                                &(*index_expr).base.base.location,
                            );
                        }
                        Value::NormalizationFailed => {
                            self.report_error_type_error_data_location(
                                crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(
                                    NormalizationTooComplex::default(),
                                ),
                                &(*index_expr).base.base.location,
                            );
                            let err = NotATable { ty: expr_type };
                            self.report_error_type_error_data_location(
                                crate::type_aliases::type_error_data::TypeErrorData::NotATable(err),
                                &(*index_expr).base.base.location,
                            );
                        }
                        Value::Suppress => {}
                    }
                }
            } else if let Some(it) = get_type_id::<IntersectionType>(expr_type).as_ref() {
                let mut any_table = false;
                let mut intersection_iter =
                    TypeIterator::<IntersectionType>::type_iterator_type(it as *const _);
                let intersection_end = TypeIterator::<IntersectionType>::type_iterator_default();
                while intersection_iter.operator_ne(&intersection_end) {
                    let part = intersection_iter.operator_deref();
                    intersection_iter.operator_inc();

                    if get_table_type(part).is_some() {
                        any_table = true;
                        break;
                    }
                }

                if !any_table {
                    let err = NotATable { ty: expr_type };
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::NotATable(err),
                        &(*index_expr).base.base.location,
                    );
                }
            } else if get_type_id::<crate::records::never_type::NeverType>(expr_type)
                .as_ref()
                .is_some()
                || self.is_error_suppressing_location_type_id(
                    (*index_expr).base.base.location,
                    expr_type,
                )
            {
                // Nothing
            } else {
                let err = NotATable { ty: expr_type };
                self.report_error_type_error_data_location(
                    crate::type_aliases::type_error_data::TypeErrorData::NotATable(err),
                    &(*index_expr).base.base.location,
                );
            }
        }
    }
}
