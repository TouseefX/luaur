use crate::records::ac_fixture::AcFixture;
use luaur_analysis::records::frontend::Frontend;

impl AcFixture {
    pub fn get_frontend(&mut self) -> &mut Frontend {
        self.base.get_frontend()
    }
}
