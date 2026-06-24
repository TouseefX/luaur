use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_variadic_type_pack::is_variadic;
use crate::functions::try_get_type_name_in_scope_autocomplete_core::try_get_type_name_in_scope;
use crate::functions::try_get_type_name_in_scope_autocomplete_core_alt_b::try_get_type_name_in_scope_scope_ptr_type_pack_id_bool;
use crate::records::function_type::FunctionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;

pub fn make_anonymous(scope: &ScopePtr, func_ty: &FunctionType) -> String {
    let mut result = String::from("function(");

    let (args, tail) = flatten_type_pack_id(func_ty.arg_types);

    let mut first = true;
    for arg_idx in 0..args.len() {
        if !first {
            result += ", ";
        } else {
            first = false;
        }

        let name: String;
        if arg_idx < func_ty.arg_names.len() {
            if let Some(ref arg_name) = func_ty.arg_names[arg_idx] {
                name = arg_name.name.clone();
            } else {
                name = format!("a{}", arg_idx);
            }
        } else {
            name = format!("a{}", arg_idx);
        }

        if let Some(ref r#type) = try_get_type_name_in_scope(scope.clone(), args[arg_idx], true) {
            result += &format!("{}: {}", name, r#type);
        } else {
            result += &name;
        }
    }

    if let Some(tail_tp) = tail {
        let followed_tail = unsafe { follow_type_pack_id(tail_tp) };
        if is_variadic(tail_tp) {
            let pack_ptr = unsafe { get_type_pack_id::<VariadicTypePack>(followed_tail) };
            if !pack_ptr.is_null() {
                let pack = unsafe { &*pack_ptr };
                if !first {
                    result += ", ";
                }
                if let Some(ref var_arg_type) =
                    try_get_type_name_in_scope(scope.clone(), pack.ty, true)
                {
                    result += &format!("...: {}", var_arg_type);
                } else {
                    result += "...";
                }
            }
        }
    }

    result += ")";

    let (rets, ret_tail) = flatten_type_pack_id(func_ty.ret_types);
    let total_ret_size = rets.len() + if ret_tail.is_some() { 1 } else { 0 };
    if total_ret_size > 0 {
        if let Some(ref return_types) = try_get_type_name_in_scope_scope_ptr_type_pack_id_bool(
            scope.clone(),
            func_ty.ret_types,
            true,
        ) {
            result += ": ";
            let wrap = total_ret_size != 1;
            if wrap {
                result += "(";
            }
            result += return_types;
            if wrap {
                result += ")";
            }
        }
    }

    result += "  end";
    result
}
