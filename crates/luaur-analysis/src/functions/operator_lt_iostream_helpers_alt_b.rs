use core::fmt::Write;
use luaur_ast::records::location::Location;

pub fn operator_lt_ostream_location(
    stream: &mut dyn Write,
    location: &Location,
) -> core::fmt::Result {
    write!(stream, "Location {{ ")?;
    crate::functions::operator_lt_iostream_helpers::operator_lt_ostream_position(
        stream,
        &location.begin,
    )?;
    write!(stream, ", ")?;
    crate::functions::operator_lt_iostream_helpers::operator_lt_ostream_position(
        stream,
        &location.end,
    )?;
    write!(stream, " }}")
}
