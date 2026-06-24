//! Generated skeleton item.
//! Node: `cxx:Method:Luau.Config:Config/include/Luau/Config.h:26:config_config`
//! Source: `Config/include/Luau/Config.h`
//! Graph edges:
//! - declared_by: source_file Config/include/Luau/Config.h
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Config/include/Luau/LinterConfig.h
//!   - includes -> source_file Ast/include/Luau/ParseOptions.h
//! - incoming:
//!   - declares <- source_file Config/include/Luau/Config.h
//!   - type_ref <- record Config (Config/include/Luau/Config.h)
//! - outgoing:
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - translates_to -> rust_item Config::Config

use crate::records::config::Config;

pub fn config_config() -> Config {
    Config::default()
}
