use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_or_pack::TypeOrPackMember;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeCloner {
    pub fn clone_children_type_or_pack(&mut self, kind: TypeOrPack) {
        if let Some(ty) = TypeId::get_if(&kind) {
            self.clone_children_type_id(*ty);
        } else if let Some(tp) = TypePackId::get_if(&kind) {
            self.clone_children_type_pack_id(*tp);
        } else {
            LUAU_ASSERT!(false);
        }
    }
}
