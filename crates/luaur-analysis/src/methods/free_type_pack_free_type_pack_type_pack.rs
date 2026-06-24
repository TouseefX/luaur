use crate::functions::fresh_index::fresh_index;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_level::TypeLevel;

impl FreeTypePack {
    pub fn free_type_pack_type_level(&mut self, level: TypeLevel) {
        self.index = fresh_index();
        self.level = level;
        self.scope = core::ptr::null_mut();
    }
}
