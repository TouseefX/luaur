use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::weird_iter::WeirdIter;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl WeirdIter {
    pub fn weird_iter_grow(&mut self, new_tail: TypePackId) {
        LUAU_ASSERT!(self.weird_iter_can_grow());
        LUAU_ASSERT!(!unsafe { get_mutable_type_pack_id::<TypePack>(new_tail) }.is_null());

        let free_pack = unsafe { get_mutable_type_pack_id::<FreeTypePack>(self.pack_id) };
        self.level = unsafe { (*free_pack).level };
        if !unsafe { (*free_pack).scope }.is_null() {
            self.scope = unsafe { (*free_pack).scope };
        }
        unsafe {
            (*self.log).replace_type_pack_id_type_pack_var(
                self.pack_id,
                TypePackVar {
                    ty: TypePackVariant::Bound(new_tail),
                    persistent: false,
                    owningArena: core::ptr::null_mut(),
                },
            );
        }
        self.pack_id = new_tail;
        self.pack = unsafe { get_mutable_type_pack_id::<TypePack>(new_tail) };
        self.index = 0;
        self.growing = true;
    }
}
