use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

impl TypeCacher {
    pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        LUAU_ASSERT!(false);
        LUAU_UNREACHABLE!();
    }
}
