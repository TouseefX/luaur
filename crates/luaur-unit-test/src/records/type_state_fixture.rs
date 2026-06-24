use crate::records::builtins_fixture::BuiltinsFixture;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use luaur_common::FFlag;

#[derive(Debug)]
pub struct TypeStateFixture {
    pub dcr: ScopedFastFlag,
    pub base: BuiltinsFixture,
}

impl Default for TypeStateFixture {
    fn default() -> Self {
        let dcr = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
        let mut base = BuiltinsFixture::default();
        base.get_frontend();

        Self { dcr, base }
    }
}
