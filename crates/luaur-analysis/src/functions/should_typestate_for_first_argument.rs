use luaur_ast::records::ast_expr_call::AstExprCall;

pub fn should_typestate_for_first_argument(call: &AstExprCall) -> bool {
    // TODO: magic function for setmetatable and assert and then add them
    crate::functions::match_table_freeze::match_table_freeze(call)
}
