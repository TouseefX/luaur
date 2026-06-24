//! @interface-stub
use crate::records::unifier_2_fixture::Unifier2Fixture;
use alloc::string::String;
use luaur_analysis::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl Unifier2Fixture {
    pub fn to_string_type_pack_id(&mut self, ty: TypePackId) -> String {
        to_string_type_pack_id_to_string_options(ty, &mut self.opts)
    }
}
