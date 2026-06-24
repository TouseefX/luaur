use crate::records::fixture::Fixture;
use luaur_analysis::records::builtin_types::BuiltinTypes;

impl Fixture {
    pub fn get_builtins(&mut self) -> &mut BuiltinTypes {
        if self.builtin_types.is_null() {
            self.get_frontend();
        }

        unsafe { &mut *self.builtin_types }
    }
}
