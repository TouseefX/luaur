use crate::records::find_free_types::FindFreeTypes;
use luaur_analysis::records::free_type::FreeType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl FindFreeTypes {
    pub fn operator_call_mut<T>(&mut self, _id: T) -> bool {
        self.found_one = true;
        false
    }
}
