use crate::enums::block_kind::BlockKind;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn block_kind_name(kind: BlockKind) -> &'static str {
    match kind {
        BlockKind::Entry => "entry",
        BlockKind::Linear => "linear",
        BlockKind::Condition => "condition",
        _ => {
            LUAU_ASSERT!(false);
            "?"
        }
    }
}
