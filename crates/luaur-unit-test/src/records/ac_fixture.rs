use crate::records::ac_fixture_impl::AcFixtureImpl;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct AcFixture {
    pub base: AcFixtureImpl,
}

impl Default for AcFixture {
    fn default() -> Self {
        Self {
            base: AcFixtureImpl::default(),
        }
    }
}
