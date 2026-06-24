use crate::records::generic_type_finder::GenericTypeFinder;
use crate::records::generic_type_pack::GenericTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl GenericTypeFinder {
    pub fn visit_type_pack_id_luau(&mut self, _ty: TypePackId, _gtp: &GenericTypePack) -> bool {
        self.found = true;
        false
    }
}
