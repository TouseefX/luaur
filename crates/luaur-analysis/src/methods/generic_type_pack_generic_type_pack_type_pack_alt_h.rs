use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type_pack::GenericTypePack;
use crate::type_aliases::name_type::Name;

impl GenericTypePack {
    pub fn generic_type_pack_polarity(&mut self, polarity: Polarity) {
        self.index = fresh_index();
        self.name = Name::from(format!("g{}", self.index).as_str());
        self.polarity = polarity;
    }
}
