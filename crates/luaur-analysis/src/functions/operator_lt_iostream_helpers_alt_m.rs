use crate::functions::to_string_type_path::to_string_type_path_path_bool;
use crate::records::path::Path;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn Write, path: &Path) -> core::fmt::Result {
    let s = to_string_type_path_path_bool(path, false);
    write!(stream, "{}", s)
}

#[allow(unused_imports, non_snake_case)]
pub use operator_lt as operator_lt_ostream_path;
