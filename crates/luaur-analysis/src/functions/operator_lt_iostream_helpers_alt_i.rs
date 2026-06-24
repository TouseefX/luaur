use crate::functions::to_string_to_string_alt_f::to_string_type_item;
use crate::records::r#type::Type;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn Write, tv: &Type) -> core::fmt::Result {
    let s = to_string_type_item(tv);
    write!(stream, "{}", s)
}

#[allow(unused_imports, non_snake_case)]
pub use operator_lt as operator_lt_ostream_type_item;
