use crate::records::find_free_types::FindFreeTypes;
use luaur_analysis::records::free_type_pack::FreeTypePack;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl FindFreeTypes {
    pub fn operator_call_mut_2(&mut self, _id: TypePackId, _free: FreeTypePack) -> bool {
        self.found_one = true;
        false
    }
}
