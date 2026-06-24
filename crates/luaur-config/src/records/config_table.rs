//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Config:Config/include/Luau/LuauConfig.h:69:config_table`
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
//!   - type_ref <- record ConfigValue (Config/include/Luau/LuauConfig.h)
//!   - type_ref <- function serializeTable (Config/src/LuauConfig.cpp)
//!   - type_ref <- function extractConfig (Config/src/LuauConfig.cpp)
//!   - type_ref <- function createLuauConfigFromLuauTable (Config/src/LuauConfig.cpp)
//!   - type_ref <- function extractLuauConfig (Config/src/LuauConfig.cpp)
//!   - type_ref <- method ConfigTable::ConfigTable (Config/include/Luau/LuauConfig.h)
//! - outgoing:
//!   - type_ref -> method ConfigTable::ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - type_ref -> record ConfigTableKey (Config/include/Luau/LuauConfig.h)
//!   - type_ref -> record ConfigValue (Config/include/Luau/LuauConfig.h)
//!   - type_ref -> record VariantHashDefault (Config/include/Luau/LuauConfig.h)
//!   - translates_to -> rust_item ConfigTable

use alloc::string::String;

use luaur_common::records::dense_hash_map::DenseHashMap;

use crate::records::config_table_key::ConfigTableKey;
use crate::records::config_value::ConfigValue;
use crate::records::variant_hash_default::VariantHashDefault;

#[derive(Debug, Clone)]
pub struct ConfigTable {
    pub map: DenseHashMap<ConfigTableKey, ConfigValue, VariantHashDefault>,
}

impl ConfigTable {
    pub fn new() -> Self {
        Self {
            map: DenseHashMap::new(ConfigTableKey::default()),
        }
    }

    pub fn get_or_insert(&mut self, key: ConfigTableKey) -> &mut ConfigValue {
        self.map.get_or_insert(key)
    }

    pub fn insert(&mut self, key: ConfigTableKey, value: ConfigValue) {
        *self.get_or_insert(key) = value;
    }

    pub fn find(&self, key: &ConfigTableKey) -> Option<&ConfigValue> {
        self.map.find(key)
    }

    pub fn find_str(&self, key: &str) -> Option<&ConfigValue> {
        self.find(&ConfigTableKey::from(key))
    }

    pub fn contains_str(&self, key: &str) -> bool {
        self.map.contains(&ConfigTableKey::from(key))
    }

    pub fn size(&self) -> usize {
        self.map.size()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ConfigTableKey, &ConfigValue)> {
        self.map.iter()
    }
}

impl Default for ConfigTable {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for ConfigTableKey {
    fn from(value: String) -> Self {
        Self(luaur_common::records::variant::Variant2::V0(value))
    }
}

impl From<&str> for ConfigTableKey {
    fn from(value: &str) -> Self {
        Self::from(String::from(value))
    }
}

impl From<f64> for ConfigTableKey {
    fn from(value: f64) -> Self {
        Self(luaur_common::records::variant::Variant2::V1(value))
    }
}
