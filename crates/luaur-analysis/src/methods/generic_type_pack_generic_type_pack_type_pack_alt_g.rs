use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;
use crate::type_aliases::name_type::Name;

impl GenericTypePack {
    pub fn generic_type_pack_scope_name_polarity(
        &mut self,
        scope: *mut Scope,
        name: Name,
        polarity: Polarity,
    ) {
        self.index = fresh_index();
        self.scope = scope;
        self.name = name;
        self.explicitName = true;
        self.polarity = polarity;
    }
}
