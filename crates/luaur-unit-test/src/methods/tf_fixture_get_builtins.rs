use crate::records::tf_fixture::TfFixture;
use luaur_analysis::records::builtin_types::BuiltinTypes;

impl TfFixture {
    pub fn get_builtins(&mut self) -> &mut BuiltinTypes {
        &mut self.builtin_types
    }
}
