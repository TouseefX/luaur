use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_function::AstExprFunction;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PushFunctionTypeConstraint {
    pub(crate) expected_function_type: TypeId,
    pub(crate) function_type: TypeId,
    pub(crate) expr: *mut AstExprFunction,
    pub(crate) is_self: bool,
}
