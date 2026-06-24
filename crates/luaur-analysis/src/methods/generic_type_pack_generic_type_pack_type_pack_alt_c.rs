use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type_pack::GenericTypePack;
use crate::type_aliases::name_type::Name;

impl GenericTypePack {
    pub fn generic_type_pack_name(&mut self, name: &Name) {
        self.index = fresh_index();
        self.name = name.clone();
        self.explicitName = true;
    }
}
