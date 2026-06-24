use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;

impl FreeTypePack {
    pub fn free_type_pack_scope_polarity(&mut self, scope: *mut Scope, polarity: Polarity) {
        self.index = fresh_index();
        self.level = TypeLevel::default();
        self.scope = scope;
        self.polarity = polarity;
    }
}
