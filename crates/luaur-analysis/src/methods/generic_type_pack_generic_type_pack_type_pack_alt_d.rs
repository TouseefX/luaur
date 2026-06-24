use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;

impl GenericTypePack {
    pub fn generic_type_pack_scope_polarity(&mut self, scope: *mut Scope, polarity: Polarity) {
        self.index = fresh_index();
        self.scope = scope;
        self.polarity = polarity;
    }
}
