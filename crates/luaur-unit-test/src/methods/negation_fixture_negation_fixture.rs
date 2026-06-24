use crate::functions::register_hidden_types::register_hidden_types;
use crate::records::negation_fixture::NegationFixture;

impl NegationFixture {
    pub fn negation_fixture_negation_fixture(&mut self) {
        register_hidden_types(&mut self.base.get_frontend());
    }
}
