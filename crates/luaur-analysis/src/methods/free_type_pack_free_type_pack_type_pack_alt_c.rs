use crate::records::free_type_pack::FreeTypePack;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;

impl FreeTypePack {
    pub fn free_type_pack_scope_type_level(&mut self, _scope: *mut Scope, _level: TypeLevel) {
        self.free_type_pack_type_level(_level);
        self.free_type_pack_scope_polarity(_scope, self.polarity);
    }
}
