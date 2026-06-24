use crate::functions::check_require_path::check_require_path;
use crate::records::generic_error::GenericError;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::sync::Arc;
use alloc::vec;
use luaur_ast::records::ast_expr_call::AstExprCall;

pub fn magic_require_handle_old_solver(
    typechecker: &mut TypeChecker,
    scope: &ScopePtr,
    expr: &AstExprCall,
    _with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    let module = typechecker.current_module.as_ref()?.clone();
    let arena = unsafe {
        &mut (*(Arc::as_ptr(&module) as *mut crate::records::module::Module)).internal_types
    };

    if expr.args.size != 1 {
        typechecker.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::GenericError(GenericError::new("require takes 1 argument".to_string())),
        );
        return None;
    }

    let arg = unsafe { *expr.args.data.add(0) };
    if !check_require_path(typechecker, arg) {
        return None;
    }

    let module_info = unsafe {
        ((*typechecker.resolver).vtable.resolve_module_info)(
            typechecker.resolver,
            &module.name,
            arg,
        )
    };

    if let Some(module_info) = module_info {
        let require_type = typechecker.check_require(scope, &module_info, &expr.base.base.location);
        return Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
            TypePack {
                head: vec![require_type],
                tail: None,
            },
        )));
    }

    None
}
