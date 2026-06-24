use crate::records::fixture::Fixture;
use alloc::vec::Vec;
use luaur_analysis::functions::to_string_error::to_string_type_error;
use luaur_analysis::records::type_error::TypeError;

impl Fixture {
    pub fn validate_errors(&mut self, errors: &Vec<TypeError>) {
        for error in errors {
            let _ = to_string_type_error(error);
        }
    }
}
