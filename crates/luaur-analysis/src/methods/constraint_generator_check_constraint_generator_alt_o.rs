use crate::enums::table_state::TableState;
use crate::functions::add_all_as_reverse_dependencies::add_all_as_reverse_dependencies;
use crate::functions::checkpoint::checkpoint;
use crate::functions::follow_type::follow_type_id;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::inference::Inference;
use crate::records::module::Module;
use crate::records::property_type::Property;
use crate::records::push_type_constraint::PushTypeConstraint;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_common::{FFlag, FInt};

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_table_optional_type_id(
        &mut self,
        scope: &ScopePtr,
        expr: *mut AstExprTable,
        expected_type: Option<TypeId>,
    ) -> Inference {
        let _in_context = InConditionalContext::new(
            &mut self.type_context,
            crate::enums::type_context::TypeContext::Default,
        );

        unsafe {
            let table_ty = (*self.arena).add_type(TableType::table_type());
            let ttv = get_mutable_type_id::<TableType>(table_ty);
            luaur_common::LUAU_ASSERT!(!ttv.is_null());

            (*ttv).state = TableState::Unsealed;
            if let Some(module) = &self.module {
                (*ttv).definition_module_name = module.name.clone();
            }
            (*ttv).definition_location = (*expr).base.base.location;
            (*ttv).scope = scope.as_ref() as *const _ as *mut _;

            let primitive_limit = FInt::LuauPrimitiveInferenceInTableLimit.get();
            let large_table = primitive_limit > 0 && (*expr).items.size > primitive_limit as usize;
            if large_table {
                self.large_table_depth += 1;
            }

            if let Some(interior) = self.interior_free_types.last_mut() {
                interior.types.push(table_ty);
            }

            let mut index_key_lower_bound: Vec<TypeId> = Vec::new();
            let mut index_value_lower_bound: Vec<TypeId> = Vec::new();

            let mut create_indexer = |current_index_type: TypeId, current_result_type: TypeId| unsafe {
                let key = follow_type_id(current_index_type);
                if !index_key_lower_bound.iter().any(|&ty| ty == key) {
                    index_key_lower_bound.push(key);
                }

                let value = follow_type_id(current_result_type);
                if !index_value_lower_bound.iter().any(|&ty| ty == value) {
                    index_value_lower_bound.push(value);
                }
            };

            let start = checkpoint(self as *const ConstraintGenerator);

            for i in 0..(*expr).items.size {
                let item = &*(*expr).items.data.add(i);
                let item_ty = self
                    .check_scope_ptr_ast_expr_optional_type_id_bool_bool(
                        scope, item.value, None, false, false,
                    )
                    .ty;

                if !item.key.is_null() {
                    let key_ty = self.check_scope_ptr_ast_expr(scope, item.key).ty;
                    let key = ast_node_as::<AstExprConstantString>(item.key as *mut AstNode);

                    if !key.is_null() {
                        let value = (*key).value;
                        let bytes = core::slice::from_raw_parts(
                            value.data as *const u8,
                            value.size as usize,
                        );
                        let prop_name =
                            alloc::string::String::from(core::str::from_utf8(bytes).unwrap_or(""));
                        let mut prop = Property::rw_type_id(item_ty);
                        prop.location = Some((*key).base.base.location);
                        (*ttv).props.insert(prop_name, prop);
                    } else {
                        create_indexer(key_ty, item_ty);
                    }
                } else {
                    create_indexer((*self.builtin_types).numberType, item_ty);
                }
            }

            let end = checkpoint(self as *const ConstraintGenerator);

            if !index_key_lower_bound.is_empty() {
                luaur_common::LUAU_ASSERT!(!index_value_lower_bound.is_empty());

                let index_key = if index_key_lower_bound.len() == 1 {
                    index_key_lower_bound[0]
                } else {
                    self.make_union_vector_type_id(index_key_lower_bound)
                };

                let index_value = if index_value_lower_bound.len() == 1 {
                    index_value_lower_bound[0]
                } else {
                    self.make_union_vector_type_id(index_value_lower_bound)
                };

                (*ttv).indexer = Some(TableIndexer {
                    index_type: index_key,
                    index_result_type: index_value,
                    is_read_only: false,
                });
            }

            if let Some(expected_type) = expected_type {
                if let Some(module) = &self.module {
                    let module_ptr = Arc::as_ptr(module) as *mut Module;
                    let ptc = self.add_constraint_scope_ptr_location_constraint_v(
                        scope,
                        (*expr).base.base.location,
                        ConstraintV::PushType(PushTypeConstraint {
                            expectedType: expected_type,
                            targetType: table_ty,
                            astTypes: &(*module_ptr).ast_types as *const _,
                            astExpectedTypes: &(*module_ptr).ast_expected_types as *const _,
                            expr: expr as *const _,
                        }),
                    );

                    if FFlag::LuauConstraintGraph.get() {
                        add_all_as_reverse_dependencies(start, end, self, ptc);
                    } else {
                        for_each_constraint(start, end, self, |c| {
                            (*c).deprecated_dependencies.push(ptc);
                        });
                    }
                }
            }

            if large_table {
                self.large_table_depth -= 1;
            }

            Inference::inference_type_id_refinement_id(table_ty, core::ptr::null_mut())
        }
    }
}
