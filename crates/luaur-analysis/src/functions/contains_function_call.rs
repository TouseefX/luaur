use crate::records::contains_function_call::ContainsFunctionCall;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::visit::ast_stat_visit;

pub fn contains_function_call(stat: &AstStat) -> bool {
    let mut cfc = ContainsFunctionCall::new(false);
    let stat_ptr = stat as *const AstStat as *mut AstStat;
    unsafe {
        ast_stat_visit(stat_ptr, &mut cfc);
    }
    cfc.result
}
