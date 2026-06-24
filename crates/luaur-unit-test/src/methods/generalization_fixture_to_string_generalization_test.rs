use crate::records::generalization_fixture::GeneralizationFixture;
use alloc::string::String;
use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use luaur_analysis::type_aliases::type_id::TypeId;

impl GeneralizationFixture {
    pub fn to_string_type_id(&mut self, ty: TypeId) -> String {
        to_string_type_id_to_string_options(ty, &mut self.opts)
    }
}
