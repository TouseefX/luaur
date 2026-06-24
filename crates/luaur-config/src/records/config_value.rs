//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Config:Config/include/Luau/LuauConfig.h:77:config_value`
//! Source: `Config/include/Luau/LuauConfig.h`
//! Graph edges:
//! - declared_by: source_file Config/include/Luau/LuauConfig.h
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file Config/include/Luau/LuauConfig.h
//!   - type_ref <- record ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - type_ref <- method ConfigTable::ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - type_ref <- function createLuauConfigFromLuauTable (Config/src/LuauConfig.cpp)
//! - outgoing:
//!   - type_ref -> record Variant (Common/include/Luau/Variant.h)
//!   - type_ref -> record ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - translates_to -> rust_item ConfigValue

use alloc::string::String;

use luaur_common::records::dense_hash_table::DenseDefault;
use luaur_common::records::variant::Variant4;

use crate::records::config_table::ConfigTable;

#[derive(Debug, Clone)]
pub struct ConfigValue(pub Variant4<String, f64, bool, ConfigTable>);

impl ConfigValue {
    pub fn get_string(&self) -> Option<&String> {
        self.0.get_if_0()
    }

    pub fn get_number(&self) -> Option<&f64> {
        self.0.get_if_1()
    }

    pub fn get_bool(&self) -> Option<&bool> {
        self.0.get_if_2()
    }

    pub fn get_table(&self) -> Option<&ConfigTable> {
        self.0.get_if_3()
    }

    pub fn get_table_mut(&mut self) -> Option<&mut ConfigTable> {
        self.0.get_if_3_mut()
    }
}

impl Default for ConfigValue {
    fn default() -> Self {
        Self(Variant4::V0(String::new()))
    }
}

impl DenseDefault for ConfigValue {
    fn dense_default() -> Self {
        Self::default()
    }
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        Self(Variant4::V0(value))
    }
}

impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        Self(Variant4::V1(value))
    }
}

impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        Self(Variant4::V2(value))
    }
}

impl From<ConfigTable> for ConfigValue {
    fn from(value: ConfigTable) -> Self {
        Self(Variant4::V3(value))
    }
}
