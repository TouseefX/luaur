use crate::records::enqueuer::Enqueuer;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn enqueuer_enqueuer() {
    let _dummy: Option<Enqueuer> = Some(Enqueuer::new(core::ptr::null_mut()));
    LUAU_ASSERT!(false);
}
