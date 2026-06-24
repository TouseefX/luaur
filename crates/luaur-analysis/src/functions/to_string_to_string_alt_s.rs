//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2130:to_string`
//! Source: `Analysis/src/ToString.cpp:2130-2133` (hand-ported)

use alloc::format;
use alloc::string::String;
use luaur_ast::records::position::Position;

/// C++ `std::string toString(const Position& position)`.
pub fn to_string_position(position: &Position) -> String {
    format!("{{ line = {}, col = {} }}", position.line, position.column)
}
