use crate::records::builtin_types::BuiltinTypes;
use crate::records::scope::Scope;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_ast::records::ast_expr_call::AstExprCall;

#[derive(Debug, Clone)]
pub struct MagicFunctionTypeCheckContext {
    pub(crate) typechecker: NonNull<TypeChecker2>,
    pub(crate) builtin_types: NonNull<BuiltinTypes>,
    pub(crate) call_site: *const AstExprCall,
    pub(crate) arguments: TypePackId,
    pub(crate) check_scope: NonNull<Scope>,
}
