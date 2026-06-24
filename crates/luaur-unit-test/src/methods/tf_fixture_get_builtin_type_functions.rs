use crate::records::tf_fixture::TfFixture;
use luaur_analysis::records::builtin_type_functions::BuiltinTypeFunctions;

impl TfFixture {
    pub fn get_builtin_type_functions(&mut self) -> &BuiltinTypeFunctions {
        &self.builtin_types.typeFunctions
    }
}
