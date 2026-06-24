use crate::type_aliases::error::Error;
use luaur_ast::enums::mode::Mode;

#[allow(non_snake_case)]
pub fn parse_mode_string(mode: &mut Mode, mode_string: &str, compat: bool) -> Error {
    if mode_string == "nocheck" {
        *mode = Mode::NoCheck;
    } else if mode_string == "strict" {
        *mode = Mode::Strict;
    } else if mode_string == "nonstrict" {
        *mode = Mode::Nonstrict;
    } else if mode_string == "noinfer" && compat {
        *mode = Mode::NoCheck;
    } else {
        return Some(alloc::format!(
            "Bad mode \"{}\".  Valid options are nocheck, nonstrict, and strict",
            mode_string
        ));
    }

    None
}
