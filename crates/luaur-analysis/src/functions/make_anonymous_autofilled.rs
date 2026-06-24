use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_variadic_type_pack::is_variadic;
use crate::functions::return_first_nonnull_option_of_type::return_first_nonnull_option_of_type;
use crate::functions::try_get_type_name_in_scope_autocomplete_core::try_get_type_name_in_scope;
use crate::functions::try_get_type_name_in_scope_autocomplete_core_alt_b::try_get_type_name_in_scope_scope_ptr_type_pack_id_bool;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::union_type::UnionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::string::{String, ToString};
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;

pub fn make_anonymous_autofilled(
    module: &ModulePtr,
    scope_at_position: &ScopePtr,
    position: Position,
    node: *const AstNode,
    ancestry: &alloc::vec::Vec<*mut AstNode>,
) -> Option<AutocompleteEntry> {
    let node_ptr = node as *mut AstNode;
    let call = unsafe { ast_node_as::<AstExprCall>(node_ptr) };
    let mut call = if call.is_null() && ancestry.len() > 1 {
        let prev_node = ancestry[ancestry.len() - 2] as *mut AstNode;
        unsafe { ast_node_as::<AstExprCall>(prev_node) }
    } else {
        call
    };

    if call.is_null() {
        return None;
    }

    let call_ref = unsafe { &*call };
    let call_loc = call_ref.base.base.location;
    let func_loc = unsafe { (*call_ref.func).base.location };
    if !call_loc.containsClosed(position) || func_loc.containsClosed(position) {
        return None;
    }

    let type_iter = module
        .ast_types
        .find(&(call_ref.func as *const luaur_ast::records::ast_expr::AstExpr))?;

    let outer_function = unsafe {
        let followed = follow_type_id(*type_iter);
        get_type_id::<FunctionType>(followed)
    };
    let outer_function = if outer_function.is_null() {
        return None;
    } else {
        unsafe { &*outer_function }
    };

    let mut argument: usize = 0;
    let args_arr = &call_ref.args;
    for i in 0..args_arr.size as usize {
        let arg_expr = unsafe { *args_arr.data.add(i) };
        let arg_loc = unsafe { (*arg_expr).base.location };
        if arg_loc.containsClosed(position) {
            argument = i;
            break;
        }
    }

    if call_ref.self_ {
        argument += 1;
    }

    let (args, _) = flatten_type_pack_id(outer_function.arg_types);
    let arg_type = if argument < args.len() {
        Some(args[argument])
    } else {
        None
    }?;

    let followed = unsafe { follow_type_id(arg_type) };
    let type_ = unsafe { get_type_id::<FunctionType>(followed) };
    let type_ = if !type_.is_null() {
        unsafe { &*type_ }
    } else {
        let union_type = unsafe { get_type_id::<UnionType>(followed) };
        if union_type.is_null() {
            return None;
        } else {
            let union_ref = unsafe { &*union_type };
            let nonnull_func =
                unsafe { return_first_nonnull_option_of_type::<FunctionType>(union_ref) };
            if let Some(nonnull_func) = nonnull_func {
                unsafe { &*nonnull_func }
            } else {
                return None;
            }
        }
    };

    let mut entry = AutocompleteEntry {
        kind: AutocompleteEntryKind::GeneratedFunction,
        r#type: Some(arg_type),
        deprecated: false,
        wrong_index_type: false,
        type_correct: TypeCorrectKind::Correct,
        containing_extern_type: None,
        prop: None,
        documentation_symbol: None,
        tags: Default::default(),
        parens: Default::default(),
        insert_text: Some(make_anonymous_local(scope_at_position, type_)),
        indexed_with_self: false,
    };

    Some(entry)
}

fn make_anonymous_local(scope: &ScopePtr, func_ty: &FunctionType) -> String {
    let mut result = String::from("function(");

    let (args, tail) = flatten_type_pack_id(func_ty.arg_types);

    let mut first_arg = true;
    for arg_idx in 0..args.len() {
        if !first_arg {
            result.push_str(", ");
        } else {
            first_arg = false;
        }

        let name = if arg_idx < func_ty.arg_names.len() {
            if let Some(arg_name) = &func_ty.arg_names[arg_idx] {
                arg_name.name.clone()
            } else {
                "a".to_string() + &arg_idx.to_string()
            }
        } else {
            "a".to_string() + &arg_idx.to_string()
        };

        if let Some(type_name) = try_get_type_name_in_scope(scope.clone(), args[arg_idx], true) {
            result.push_str(&name);
            result.push_str(": ");
            result.push_str(&type_name);
        } else {
            result.push_str(&name);
        }
    }

    if let Some(tail_id) = tail {
        let followed_tail = unsafe { follow_type_pack_id(tail_id) };
        let free_tail = unsafe { get_type_pack_id::<FreeTypePack>(followed_tail) };
        if is_variadic(tail_id) || !free_tail.is_null() {
            if !first_arg {
                result.push_str(", ");
            }

            let mut var_arg_type = None;
            let pack = unsafe { get_type_pack_id::<VariadicTypePack>(followed_tail) };
            if !pack.is_null() {
                var_arg_type =
                    try_get_type_name_in_scope(scope.clone(), unsafe { (*pack).ty }, true);
            }

            if let Some(var_arg_type) = var_arg_type {
                result.push_str("...: ");
                result.push_str(&var_arg_type);
            } else {
                result.push_str("...");
            }
        }
    }

    result.push(')');

    let (rets, ret_tail) = flatten_type_pack_id(func_ty.ret_types);
    let total_ret_size = rets.len() + if ret_tail.is_some() { 1 } else { 0 };
    if total_ret_size > 0 {
        if let Some(return_types) = try_get_type_name_in_scope_scope_ptr_type_pack_id_bool(
            scope.clone(),
            func_ty.ret_types,
            true,
        ) {
            result.push_str(": ");
            let wrap = total_ret_size != 1;
            if wrap {
                result.push('(');
            }
            result.push_str(&return_types);
            if wrap {
                result.push(')');
            }
        }
    }

    result.push_str("  end");
    result
}
