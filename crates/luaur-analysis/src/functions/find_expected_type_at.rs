use crate::functions::first::first;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_variadic_type_pack::is_variadic;
use crate::records::function_type::FunctionType;
use crate::records::module::Module;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::{ast_node_as, ast_node_is};

pub fn find_expected_type_at(
    module: &Module,
    node: *mut AstNode,
    position: Position,
) -> Option<TypeId> {
    let expr = unsafe { (*node).as_expr_const() as *mut AstExpr };
    if expr.is_null() {
        return None;
    }

    // Extra care for first function call argument location
    // When we don't have anything inside () yet, we also don't have an AST node to base our lookup
    if unsafe { ast_node_is::<AstExprCall>(&(*expr).base) } {
        let expr_call = unsafe { ast_node_as::<AstExprCall>(expr as *mut AstNode) };
        if !expr_call.is_null() {
            let args_size = unsafe { (*expr_call).args.size };
            let arg_location = unsafe { (*expr_call).arg_location };

            if (args_size == 0 && arg_location.contains(position))
                || (args_size > 0
                    && unsafe {
                        let first_arg = *(*expr_call).args.data;
                        ast_node_is::<AstExprError>(&(*first_arg).base)
                    })
            {
                let it = module
                    .ast_types
                    .find(&(unsafe { (*expr_call).func } as *const AstExpr));
                if it.is_none() {
                    return None;
                }

                let follow_ty = unsafe { follow_type_id(*it.unwrap()) };
                let ftv_ptr = unsafe { get_type_id::<FunctionType>(follow_ty) };
                if ftv_ptr.is_null() {
                    return None;
                }

                let ftv = unsafe { &*ftv_ptr };
                let (head, tail) = flatten_type_pack_id(ftv.arg_types);
                let mut index = if unsafe { (*expr_call).self_ } { 1 } else { 0 };

                if index < head.len() as usize {
                    return Some(head[index as usize]);
                } else if index == head.len() as usize && tail.is_some() {
                    let tail_tp = tail.unwrap();
                    if is_variadic(tail_tp) {
                        return first(tail_tp, false);
                    }
                }

                return None;
            }
        }
    }

    let it = module.ast_expected_types.find(&(expr as *const AstExpr));
    if it.is_none() {
        return None;
    }

    Some(*it.unwrap())
}
