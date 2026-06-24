use crate::records::fixture::Fixture;
use luaur_analysis::records::builtin_type_functions::BuiltinTypeFunctions;

impl Fixture {
    pub fn get_builtin_type_functions(&mut self) -> &BuiltinTypeFunctions {
        let builtins = self.get_builtins();
        unsafe { &*(*builtins).typeFunctions }
    }
}
