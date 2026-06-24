use crate::records::type_pack_fixture::TypePackFixture;
use alloc::boxed::Box;
use luaur_analysis::records::type_pack::TypePack;
use luaur_analysis::records::type_pack_var::TypePackVar;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl TypePackFixture {
    pub fn new_type_pack(
        &mut self,
        types: alloc::vec::Vec<TypeId>,
        tail: Option<TypePackId>,
    ) -> TypePackId {
        let type_pack = Box::new(TypePackVar::from(TypePack::new(types, tail)));
        let type_pack_id = type_pack.as_ref() as *const TypePackVar;
        self.type_packs.push(type_pack);
        type_pack_id
    }
}
