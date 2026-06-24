use crate::records::builtins_fixture::BuiltinsFixture;
use luaur_analysis::type_aliases::type_id::TypeId;

#[derive(Debug)]
pub struct ExternTypeFixture {
    pub base: BuiltinsFixture,
    pub vector2_type: TypeId,
    pub vector2_instance_type: TypeId,
}

impl ExternTypeFixture {
    pub fn new(prepare_autocomplete: bool) -> Self {
        let mut base = BuiltinsFixture::default();
        base.builtins_fixture_builtins_fixture(prepare_autocomplete);
        Self {
            base,
            vector2_type: core::ptr::null(),
            vector2_instance_type: core::ptr::null(),
        }
    }
}

impl Default for ExternTypeFixture {
    fn default() -> Self {
        Self::new(false)
    }
}
