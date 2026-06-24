use crate::records::contains_function_call::ContainsFunctionCall;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::visit::ast_stat_visit;

pub fn contains_function_call_or_return(stat: &AstStat) -> bool {
    let mut cfc = ContainsFunctionCall::new(true);
    unsafe {
        let stat_ptr = stat as *const AstStat as *mut AstStat;
        ast_stat_visit(stat_ptr, &mut cfc);
    }
    cfc.result
}
