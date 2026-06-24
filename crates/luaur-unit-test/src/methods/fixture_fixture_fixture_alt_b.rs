use crate::records::fixture::Fixture;
use luaur_analysis::functions::reset_print_line::reset_print_line;

impl Fixture {
    pub fn drop(&mut self) {
        reset_print_line();
    }
}
