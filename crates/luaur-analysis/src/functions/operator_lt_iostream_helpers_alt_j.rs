use crate::functions::to_string_to_string_alt_g::to_string_type_pack_var;
use crate::records::type_pack_var::TypePackVar;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn Write, tv: &TypePackVar) -> core::fmt::Result {
    let s = to_string_type_pack_var(tv);
    write!(stream, "{}", s)
}

#[allow(unused_imports, non_snake_case)]
pub use operator_lt as operator_lt_ostream_type_pack_var;
