use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::maybe_string::maybe_string;
use crate::records::function_type::FunctionType;
use crate::records::property_type::Property;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_expr_table::ItemKind;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_expr_table(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprTable,
        field_types: &alloc::vec::Vec<(TypeId, TypeId)>,
        expected_type: Option<TypeId>,
    ) -> TypeId {
        let mut props: Props = Props::new();
        let mut indexer: Option<TableIndexer> = None;

        let mut expected_table: Option<&TableType> = None;

        if let Some(expected_type) = expected_type {
            let ttv = unsafe { get_type_id::<TableType>(follow_type_id(expected_type)) };
            if !ttv.is_null() {
                let ttv = unsafe { &*ttv };
                if ttv.state == TableState::Sealed {
                    expected_table = Some(ttv);
                }
            }
        }

        for i in 0..expr.items.size {
            let item = unsafe { &*expr.items.data.add(i) };

            let k = item.key;
            let value = item.value;

            let (key_type, value_type) = field_types[i];

            if item.kind == ItemKind::List {
                if expected_table.is_some() && indexer.is_none() {
                    indexer = expected_table.unwrap().indexer.clone();
                }

                if let Some(indexer) = indexer.clone() {
                    self.unify_type_id_type_id_scope_ptr_location(
                        self.number_type,
                        indexer.index_type,
                        scope,
                        unsafe { &(*value).base.location },
                    );
                    self.unify_type_id_type_id_scope_ptr_location(
                        value_type,
                        indexer.index_result_type,
                        scope,
                        unsafe { &(*value).base.location },
                    );
                } else {
                    let number_type = self.number_type;
                    let result_ty = self.any_if_nonstrict(value_type);
                    indexer = Some(TableIndexer {
                        index_type: number_type,
                        index_result_type: result_ty,
                        is_read_only: false,
                    });
                }
            } else if item.kind == ItemKind::Record || item.kind == ItemKind::General {
                let key = unsafe { ast_node_as::<AstExprConstantString>(k as *mut AstNode) };
                if !key.is_null() {
                    let key = unsafe { &*key };
                    let mut expr_type = unsafe { follow_type_id(value_type) };
                    if self.is_nonstrict_mode()
                        && get_table_type(expr_type).is_none()
                        && unsafe { get_type_id::<FunctionType>(expr_type) }.is_null()
                    {
                        expr_type = self.any_type;
                    }

                    let key_str: alloc::string::String = {
                        let bytes = unsafe {
                            core::slice::from_raw_parts(key.value.data as *const u8, key.value.size)
                        };
                        alloc::string::String::from(core::str::from_utf8(bytes).unwrap_or(""))
                    };

                    if let Some(expected_table) = expected_table {
                        if let Some(it) = expected_table.props.get(&key_str) {
                            let expected_prop = it.clone();
                            let errors = self.try_unify(
                                expr_type,
                                expected_prop.type_deprecated(),
                                scope,
                                unsafe { &(*k).base.location },
                            );
                            if errors.is_empty() {
                                expr_type = expected_prop.type_deprecated();
                            }
                        } else if expected_table.indexer.is_some()
                            && maybe_string(expected_table.indexer.as_ref().unwrap().index_type)
                        {
                            let index_result_type =
                                expected_table.indexer.as_ref().unwrap().index_result_type;
                            let errors =
                                self.try_unify(expr_type, index_result_type, scope, unsafe {
                                    &(*k).base.location
                                });
                            if errors.is_empty() {
                                expr_type = index_result_type;
                            }
                        }
                    }

                    props.insert(
                        key_str,
                        Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
                            expr_type,
                            false,
                            alloc::string::String::new(),
                            Some(unsafe { (*k).base.location }),
                            Default::default(),
                            None,
                            None,
                        ),
                    );
                } else {
                    if expected_table.is_some() && indexer.is_none() {
                        indexer = expected_table.unwrap().indexer.clone();
                    }

                    if let Some(indexer) = indexer.clone() {
                        self.unify_type_id_type_id_scope_ptr_location(
                            key_type,
                            indexer.index_type,
                            scope,
                            unsafe { &(*k).base.location },
                        );
                        self.unify_type_id_type_id_scope_ptr_location(
                            value_type,
                            indexer.index_result_type,
                            scope,
                            unsafe { &(*value).base.location },
                        );
                    } else if self.is_nonstrict_mode() {
                        indexer = Some(TableIndexer {
                            index_type: self.any_type,
                            index_result_type: self.any_type,
                            is_read_only: false,
                        });
                    } else {
                        indexer = Some(TableIndexer {
                            index_type: key_type,
                            index_result_type: value_type,
                            is_read_only: false,
                        });
                    }
                }
            }
        }

        let state = TableState::Unsealed;
        let mut table = TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            indexer,
            scope.level.clone(),
            state,
        );
        table.definition_module_name = self.current_module.as_ref().unwrap().name.clone();
        table.definition_location = expr.base.base.location;
        unsafe {
            let module = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            (*module).internal_types.add_type(table)
        }
    }
}
