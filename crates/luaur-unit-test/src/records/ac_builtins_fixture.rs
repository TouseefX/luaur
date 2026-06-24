use crate::records::ac_fixture_impl::AcFixtureImpl;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct ACBuiltinsFixture {
    pub base: AcFixtureImpl,
}

impl Default for ACBuiltinsFixture {
    fn default() -> Self {
        let mut base = AcFixtureImpl::default();
        base.register_builtins = true;
        Self { base }
    }
}
