use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeCloner {
    pub fn find_type_or_pack(&self, kind: TypeOrPack) -> Option<TypeOrPack> {
        if let Some(ty) = TypeOrPack::get_if::<TypeId>(&kind) {
            return self.find_type_id(*ty).map(TypeOrPack::V0);
        } else if let Some(tp) = TypeOrPack::get_if::<TypePackId>(&kind) {
            return self.find_type_pack_id(*tp).map(TypeOrPack::V1);
        } else {
            LUAU_ASSERT!(false);
            return None;
        }
    }
}
