use crate::records::apply_mapped_generics::ApplyMappedGenerics;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::functions::get_if_variant::get_if;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ApplyMappedGenerics {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let env = unsafe { &*self.env };
        let result = env.lookup_generic_pack(tp);
        if let Some(mapped_gen) = get_if::<TypePackId, _>(&result) {
            return *mapped_gen;
        }
        LUAU_ASSERT!(false);
        unsafe { (*self.builtin_types).anyTypePack }
    }
}
