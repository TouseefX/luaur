use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::demoter::Demoter;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Demoter {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let ftp = unsafe { get_type_pack_id::<FreeTypePack>(tp) };
        LUAU_ASSERT!(!ftp.is_null());
        let demoted_level = self.demoted_level(unsafe { (*ftp).level });
        let ftp_var = FreeTypePack {
            index: fresh_index(),
            level: demoted_level,
            scope: core::ptr::null_mut(),
            polarity: Polarity::Unknown,
        };

        let ty_pack_var = TypePackVar {
            ty: TypePackVariant::Free(ftp_var),
            persistent: false,
            owningArena: core::ptr::null_mut(),
        };

        unsafe { (*self.arena).add_type_pack_t(ty_pack_var) }
    }
}
