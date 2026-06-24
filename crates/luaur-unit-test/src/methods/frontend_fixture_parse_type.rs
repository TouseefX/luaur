use crate::records::frontend_fixture::FrontendFixture;
use luaur_analysis::type_aliases::type_id::TypeId;

impl FrontendFixture {
    pub fn parse_type(&mut self, src: &str) -> TypeId {
        self.base.base.parse_type(src)
    }
}
