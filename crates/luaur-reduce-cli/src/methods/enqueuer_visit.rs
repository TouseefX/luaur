use luaur_ast::records::ast_stat_block::AstStatBlock;

use crate::records::enqueuer::Enqueuer;

pub fn enqueuer_visit(this: &mut Enqueuer, block: *mut AstStatBlock) -> bool {
    unsafe {
        (*this.queue).push_back(block);
    }
    false
}
