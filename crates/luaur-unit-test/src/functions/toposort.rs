use alloc::vec::Vec;
use luaur_analysis::functions::toposort::toposort as analysis_toposort;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;

pub fn toposort(block: &mut AstStatBlock) -> Vec<*mut AstStat> {
    let mut result = Vec::with_capacity(block.body.size);

    for i in 0..block.body.size {
        let stat = unsafe { *block.body.data.add(i) };
        result.push(stat);
    }

    analysis_toposort(&mut result);

    result
}
