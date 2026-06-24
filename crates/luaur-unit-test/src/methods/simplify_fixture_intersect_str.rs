use crate::records::simplify_fixture::SimplifyFixture;
use alloc::string::String;
use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SimplifyFixture {
    pub fn intersect_str(&mut self, a: TypeId, b: TypeId) -> String {
        let ty = self.intersect(a, b);
        to_string_type_id_to_string_options(ty, &mut self.opts)
    }
}
