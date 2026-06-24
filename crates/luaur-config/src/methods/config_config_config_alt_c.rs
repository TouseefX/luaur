//! Generated skeleton item.
//! Node: `cxx:Method:Luau.Config:Config/src/Config.cpp:20:config_config`
//! Source: `Config/src/Config.cpp`
//! Graph edges:
//! - declared_by: source_file Config/src/Config.cpp
//! - source_includes:
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Ast/include/Luau/Lexer.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//! - incoming:
//!   - declares <- source_file Config/src/Config.cpp
//! - outgoing:
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - translates_to -> rust_item Config::Config

use crate::records::config::Config;

pub fn config_config(other: &Config) -> Config {
    let mut result = Config::default();
    result.config_config_mut(other);
    result
}
