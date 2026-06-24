use crate::enums::control_flow::ControlFlow;
use crate::enums::value_context::ValueContext;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_generic::is_generic;
use crate::functions::is_nil::is_nil;
use crate::functions::maybe_generic::maybe_generic;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::symbol::Symbol;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_assign(
        &mut self,
        scope: &ScopePtr,
        assign: &AstStatAssign,
    ) -> ControlFlow {
        let mut expected_types: alloc::vec::Vec<Option<TypeId>> = alloc::vec::Vec::new();
        expected_types.reserve(assign.vars.size);

        let module_scope = unsafe { (*self.current_module.as_ref().unwrap()).get_module_scope() };

        for i in 0..assign.vars.size {
            let dest = unsafe { *assign.vars.data.add(i) };

            let a_local = unsafe { ast_node_as::<AstExprLocal>(dest as *mut AstNode) };
            let a_global = unsafe { ast_node_as::<AstExprGlobal>(dest as *mut AstNode) };

            if !a_local.is_null() {
                // AstExprLocal l-values will have to be checked again because their type
                // might have been mutated during checkExprList later
                let local = unsafe { (*a_local).local };
                expected_types.push(scope.lookup_symbol(Symbol::from_local(local)));
            } else if !a_global.is_null() {
                // AstExprGlobal l-values lookup is inlined here to avoid creating a global
                // binding before checkExprList
                let name = unsafe { (*a_global).name };
                match module_scope.bindings.get(&Symbol::from_global(name)) {
                    Some(binding) => expected_types.push(Some(binding.type_id)),
                    None => expected_types.push(None),
                }
            } else {
                expected_types.push(Some(self.check_l_value(
                    scope,
                    unsafe { &*dest },
                    ValueContext::LValue,
                )));
            }
        }

        let value_pack = self
            .check_expr_list(
                scope,
                &assign.base.base.location,
                &assign.values,
                false,
                &alloc::vec::Vec::new(),
                &expected_types,
            )
            .r#type;

        let mut value_iter = begin(value_pack);
        let value_end = end(value_pack);

        let mut growing_pack: *mut TypePack = core::ptr::null_mut();

        for i in 0..assign.vars.size {
            let dest = unsafe { *assign.vars.data.add(i) };
            let left: TypeId;

            let is_local = !unsafe { ast_node_as::<AstExprLocal>(dest as *mut AstNode) }.is_null();
            let is_global =
                !unsafe { ast_node_as::<AstExprGlobal>(dest as *mut AstNode) }.is_null();

            if is_local || is_global {
                left = self.check_l_value(scope, unsafe { &*dest }, ValueContext::LValue);
            } else {
                left = expected_types[i].unwrap();
            }

            let mut right: TypeId = core::ptr::null();

            let loc = if assign.values.size == 0 {
                assign.base.base.location
            } else if i < assign.values.size {
                unsafe { (**assign.values.data.add(i)).base.location }
            } else {
                unsafe {
                    (**assign.values.data.add(assign.values.size - 1))
                        .base
                        .location
                }
            };

            if value_iter.operator_ne(&value_end) {
                right = unsafe { follow_type_id(*value_iter.operator_deref()) };
                value_iter.operator_inc();
            } else if !growing_pack.is_null() {
                unsafe { (*growing_pack).head.push(left) };
                continue;
            } else if let Some(tail) = value_iter.tail() {
                let tail_pack = unsafe { follow_type_pack_id(tail) };
                if !unsafe { get_type_pack_id::<ErrorTypePack>(tail_pack) }.is_null() {
                    right = self.error_recovery_type_scope_ptr(scope);
                } else {
                    let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(tail_pack) };
                    if !vtp.is_null() {
                        right = unsafe { (*vtp).ty };
                    } else if !unsafe { get_type_pack_id::<FreeTypePack>(tail_pack) }.is_null() {
                        unsafe {
                            (*as_mutable_type_pack_id(tail_pack)).ty =
                                TypePackVariant::TypePack(TypePack {
                                    head: alloc::vec![left],
                                    tail: None,
                                });
                        }
                        growing_pack = unsafe { get_mutable_type_pack_id::<TypePack>(tail_pack) };
                    }
                }
            }

            if !right.is_null() {
                if !luaur_common::FFlag::LuauInstantiateInSubtyping.get() {
                    if !maybe_generic(left) && is_generic(right) {
                        right = self.instantiate(
                            scope,
                            right,
                            loc,
                            crate::records::txn_log::TxnLog::empty(),
                        );
                    }
                }

                // Setting a table entry to nil doesn't mean nil is the type of the indexer,
                // it is just deleting the entry
                let mut dest_table_type_receiving_nil: *const TableType = core::ptr::null();
                let index_expr = unsafe { ast_node_as::<AstExprIndexExpr>(dest as *mut AstNode) };
                if is_nil(right) && !index_expr.is_null() {
                    let expr_ty = self
                        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                            scope,
                            unsafe { &*(*index_expr).expr },
                            None,
                            false,
                        )
                        .r#type;
                    dest_table_type_receiving_nil = match get_table_type(expr_ty) {
                        Some(t) => t as *const TableType,
                        None => core::ptr::null(),
                    };
                }

                if dest_table_type_receiving_nil.is_null()
                    || unsafe { (*dest_table_type_receiving_nil).indexer.is_none() }
                {
                    // In nonstrict mode, any assignments where the lhs is free and rhs isn't
                    // a function, we give it any type.
                    if self.is_nonstrict_mode()
                        && !unsafe { get_type_id::<FreeType>(follow_type_id(left)) }.is_null()
                        && unsafe { get_type_id::<FunctionType>(follow_type_id(right)) }.is_null()
                    {
                        self.unify_type_id_type_id_scope_ptr_location(
                            self.any_type,
                            left,
                            scope,
                            &loc,
                        );
                    } else {
                        self.unify_type_id_type_id_scope_ptr_location(right, left, scope, &loc);
                    }
                }
            }
        }

        ControlFlow::None
    }
}
