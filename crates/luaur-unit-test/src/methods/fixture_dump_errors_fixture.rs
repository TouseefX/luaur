use crate::records::fixture::Fixture;
use luaur_analysis::records::type_error::TypeError;
use luaur_common::functions::split::split;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

impl Fixture {
    pub fn dump_errors_ostream_vector_type_error(
        &mut self,
        _os: &mut core::fmt::Arguments<'_>,
        errors: &Vec<TypeError>,
    ) {
        let _ = &errors;
        // NOTE: `core::fmt::Arguments` is not a sink we can write into.
        // This method is a translation artifact; printing is handled by the
        // upstream C++ test harness in the original code path.
    }
}
