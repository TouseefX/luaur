use crate::enums::polarity::Polarity;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::scope::Scope;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Unifier2 {
    pub fn fresh_type_pack(&mut self, scope: NonNull<Scope>, polarity: Polarity) -> TypePackId {
        let result = unsafe { (*self.arena.as_ptr()).fresh_type_pack(scope.as_ptr(), polarity) };

        let ftp = unsafe { get_mutable_type_pack_id::<FreeTypePack>(result) };
        LUAU_ASSERT!(!ftp.is_null());
        unsafe { (*ftp).polarity = polarity };

        self.new_fresh_type_packs.push(result);
        result
    }
}
