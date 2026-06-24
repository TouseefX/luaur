use crate::records::generic_type_pack::GenericTypePack;
use crate::records::mapped_generic_environment::MappedGenericEnvironment;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::variant::Variant3;

impl MappedGenericEnvironment {
    pub fn bind_generic(&mut self, generic_tp: TypePackId, bindee_tp: TypePackId) -> bool {
        // We shouldn't bind generic type packs to themselves
        if generic_tp == bindee_tp {
            return true;
        }

        if unsafe {
            crate::functions::get_type_pack::get_type_pack_id::<GenericTypePack>(generic_tp)
        }
        .is_null()
        {
            LUAU_ASSERT!(false);
            return false;
        }

        let lookup_result = self.lookup_generic_pack(generic_tp);
        if let Variant3::V1(unmapped) = lookup_result {
            *self.frames[unmapped.scope_index]
                .mappings
                .get_or_insert(generic_tp) = Some(bindee_tp);
            true
        } else {
            false
        }
    }
}
