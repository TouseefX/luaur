use crate::records::fixture::Fixture;
use alloc::collections::BTreeMap;
use luaur_ast::records::position::Position;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct AcFixtureImpl {
    pub base: Fixture,
    pub marker_position: BTreeMap<core::ffi::c_char, Position>,
    pub autocomplete_globals_registered: bool,
    pub register_builtins: bool,
}

impl Default for AcFixtureImpl {
    fn default() -> Self {
        Self {
            base: Fixture::fixture_bool(true),
            marker_position: BTreeMap::new(),
            autocomplete_globals_registered: false,
            register_builtins: false,
        }
    }
}
