//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Error.cpp:1447:to_string`
//! Source: `Analysis/src/Error.cpp:1447-1450` (hand-ported)

use crate::functions::to_string_error_alt_k::to_string_type_error_type_error_to_string_options;
use crate::records::type_error::TypeError;
use crate::records::type_error_to_string_options::TypeErrorToStringOptions;
use alloc::string::String;

/// C++ `std::string toString(const TypeError& error)`.
pub fn to_string_type_error(error: &TypeError) -> String {
    to_string_type_error_type_error_to_string_options(error, TypeErrorToStringOptions::default())
}

#[allow(unused_imports)]
pub use to_string_type_error as to_string;
