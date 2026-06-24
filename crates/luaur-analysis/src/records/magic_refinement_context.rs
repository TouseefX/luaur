use crate::records::scope::Scope;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_call::AstExprCall;

#[derive(Debug, Clone)]
pub struct MagicRefinementContext {
    pub scope: *mut Scope,
    pub call_site: *const AstExprCall,
    pub discriminant_types: alloc::vec::Vec<Option<TypeId>>,
}
