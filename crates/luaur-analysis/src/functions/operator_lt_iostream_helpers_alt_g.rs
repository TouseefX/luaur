use crate::records::type_error::TypeError;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn Write, error: &TypeError) -> core::fmt::Result {
    write!(stream, "TypeError {{ \"{}\", ", error.module_name)?;
    crate::functions::operator_lt_iostream_helpers_alt_b::operator_lt_ostream_location(
        stream,
        &error.location,
    )?;
    write!(stream, ", ")?;
    crate::functions::operator_lt_iostream_helpers_alt_f::operator_lt_ostream_type_error_data(
        stream,
        &error.data,
    )?;
    write!(stream, " }}")
}
