use alloc::vec::Vec;

use luaur_ast::records::ast_stat::AstStat;

use crate::records::reducer::Reducer;

impl Reducer {
    pub fn reallocate_statements(&self, statements: &Vec<*mut AstStat>) -> Vec<*mut AstStat> {
        statements.clone()
    }
}

pub fn reducer_reallocate_statements(
    this: &Reducer,
    statements: &Vec<*mut AstStat>,
) -> Vec<*mut AstStat> {
    this.reallocate_statements(statements)
}
