use core::fmt::Write;
use luaur_ast::records::position::Position;

pub fn operator_lt_ostream_position(
    stream: &mut dyn Write,
    position: &Position,
) -> core::fmt::Result {
    // Add one so that the numbers we display match what people see in their text editors.
    write!(
        stream,
        "{{ line = {}, col = {} }}",
        position.line + 1,
        position.column + 1
    )
}
