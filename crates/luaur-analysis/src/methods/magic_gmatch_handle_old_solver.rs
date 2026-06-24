use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::parse_pattern_string::parse_pattern_string;
use crate::records::function_type::FunctionType;
use crate::records::module::Module;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn magic_gmatch_handle_old_solver(
    typechecker: &mut TypeChecker,
    scope: &ScopePtr,
    expr: &AstExprCall,
    with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    let param_pack = with_predicate.r#type;
    let (params, _tail) = flatten_type_pack_id(param_pack);

    if params.len() != 2 {
        return None;
    }

    let module = typechecker.current_module.as_ref()?;
    let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

    let index = if expr.self_ { 0 } else { 1 };
    let pattern = if expr.args.size > index {
        let arg = unsafe { *expr.args.data.add(index) };
        unsafe { ast_node_as::<AstExprConstantString>(arg as *mut AstNode) }
    } else {
        core::ptr::null_mut()
    };

    if pattern.is_null() {
        return None;
    }

    let return_types: Vec<_> = unsafe {
        parse_pattern_string(
            core::ptr::NonNull::new_unchecked(typechecker.builtin_types),
            (*pattern).value.data,
            (*pattern).value.size,
        )
    };

    if return_types.is_empty() {
        return None;
    }

    let first_location = unsafe { &(*(*expr.args.data.add(0))).base.location };
    typechecker.unify_type_id_type_id_scope_ptr_location(
        params[0],
        typechecker.string_type,
        scope,
        first_location,
    );

    let empty_pack = arena.add_type_pack_t(TypePack {
        head: Vec::new(),
        tail: None,
    });
    let return_list = arena.add_type_pack_t(TypePack {
        head: return_types,
        tail: None,
    });
    let iterator_type = arena.add_type(FunctionType::function_type_new(
        empty_pack,
        return_list,
        None,
        false,
    ));
    Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
        TypePack {
            head: vec![iterator_type],
            tail: None,
        },
    )))
}
