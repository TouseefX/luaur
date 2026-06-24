use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::instantiation_2::Instantiation2;
use crate::type_aliases::type_pack_id::TypePackId;

impl Instantiation2 {
    pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
        let generic_pack = unsafe { get_type_pack_id::<GenericTypePack>(tp) };
        !generic_pack.is_null() && self.generic_pack_substitutions.find(&tp).is_some()
    }
}
