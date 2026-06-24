use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::subtyping::Subtyping;

impl SubtypeFixture {
    pub fn mk_subtyping(&mut self) -> Subtyping {
        Subtyping::subtyping_owned(
            &mut *self.builtin_types,
            &mut *self.arena,
            &mut *self.normalizer,
            &mut *self.type_function_runtime,
            &mut *self.ice_reporter,
        )
    }
}
