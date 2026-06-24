use crate::functions::finite::finite;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_table_intersection::is_table_intersection;
use crate::functions::is_table_union::is_table_union;
use crate::records::binding::Binding;
use crate::records::cannot_extend_table::{CannotExtendTable, CannotExtendTable_Context};
use crate::records::generic_error::GenericError;
use crate::records::metatable_type::MetatableType;
use crate::records::symbol::Symbol;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn magic_set_metatable_handle_old_solver(
    typechecker: &mut TypeChecker,
    scope: &ScopePtr,
    expr: &AstExprCall,
    with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    let param_pack = with_predicate.r#type;

    if crate::functions::size_type_pack::size(param_pack, core::ptr::null_mut()) < 2
        && finite(param_pack, core::ptr::null_mut())
    {
        return None;
    }

    let module = typechecker.current_module.as_ref()?.clone();
    let arena = unsafe {
        &mut (*(Arc::as_ptr(&module) as *mut crate::records::module::Module)).internal_types
    };

    let expected_args = typechecker.un_type_pack(scope, param_pack, 2, &expr.base.base.location);
    let target = unsafe { follow_type_id(expected_args[0]) };
    let mt = unsafe { follow_type_id(expected_args[1]) };

    typechecker.tablify(target);
    typechecker.tablify(mt);

    let tab = unsafe { get_type_id::<TableType>(target) };
    if !tab.is_null() {
        let tab_ref = unsafe { &*tab };
        if unsafe { (*target).persistent } {
            typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
                expr.base.base.location,
                TypeErrorData::CannotExtendTable(CannotExtendTable {
                    table_type: target,
                    context: CannotExtendTable_Context::Metatable,
                    prop: String::new(),
                }),
            ));
        } else {
            let mt_ttv = unsafe { get_type_id::<TableType>(mt).as_ref() };
            let mut mtv = MetatableType {
                table: target,
                metatable: mt,
                syntheticName: None,
            };

            if (tab_ref.name.is_some() || tab_ref.synthetic_name.is_some())
                && mt_ttv.map_or(false, |mt_ttv| {
                    mt_ttv.name.is_some() || mt_ttv.synthetic_name.is_some()
                })
            {
                let table_name = tab_ref
                    .name
                    .as_ref()
                    .or(tab_ref.synthetic_name.as_ref())
                    .unwrap();
                let metatable_name = mt_ttv
                    .and_then(|mt_ttv| mt_ttv.name.as_ref().or(mt_ttv.synthetic_name.as_ref()))
                    .unwrap();

                if table_name == metatable_name {
                    mtv.syntheticName = Some(table_name.clone());
                }
            }

            let mt_ty = arena.add_type(mtv);

            if expr.args.size < 1 {
                return None;
            }

            if !expr.self_ {
                let target_expr = unsafe { *expr.args.data.add(0) };
                let target_local =
                    unsafe { ast_node_as::<AstExprLocal>(target_expr as *mut AstNode) };
                if !target_local.is_null() {
                    let scope_ptr = Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
                    unsafe {
                        (*scope_ptr).bindings.insert(
                            Symbol::from_local((*target_local).local),
                            Binding {
                                type_id: mt_ty,
                                location: expr.base.base.location,
                                deprecated: false,
                                deprecated_suggestion: String::new(),
                                documentation_symbol: None,
                            },
                        );
                    }
                }
            }

            return Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
                TypePack {
                    head: vec![mt_ty],
                    tail: None,
                },
            )));
        }
    } else if unsafe { !get_type_id::<crate::records::any_type::AnyType>(target).is_null() }
        || unsafe { !get_type_id::<ErrorType>(target).is_null() }
        || is_table_intersection(target)
    {
    } else if is_table_union(target) {
        let ut = unsafe { get_type_id::<UnionType>(target) };
        let ut_ref = unsafe { &*ut };

        let mut result_parts: Vec<TypeId> = Vec::new();
        for &ty in &ut_ref.options {
            result_parts.push(arena.add_type(MetatableType {
                table: ty,
                metatable: mt,
                syntheticName: None,
            }));
        }

        let result_union = arena.add_type(UnionType {
            options: result_parts,
        });
        return Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
            TypePack {
                head: vec![result_union],
                tail: None,
            },
        )));
    } else {
        typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            TypeErrorData::GenericError(GenericError::new(
                "setmetatable should take a table".to_string(),
            )),
        ));
    }

    Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
        TypePack {
            head: vec![target],
            tail: None,
        },
    )))
}
