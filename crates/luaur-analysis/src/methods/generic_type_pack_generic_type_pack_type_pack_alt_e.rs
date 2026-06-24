use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

impl GenericTypePack {
    pub fn generic_type_pack_type_level_name(&mut self, level: TypeLevel, name: &Name) {
        self.index = fresh_index();
        self.level = level;
        self.name = name.clone();
        self.explicitName = true;
    }
}
