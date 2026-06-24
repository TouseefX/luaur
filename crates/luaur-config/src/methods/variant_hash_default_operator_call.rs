use luaur_common::records::dense_hash_table::DenseHasher;
use luaur_common::records::variant::Variant2;
use luaur_common::type_aliases::dense_hash_default::DenseHashDefault;

use crate::records::config_table_key::ConfigTableKey;
use crate::records::variant_hash_default::VariantHashDefault;

impl VariantHashDefault {
    #[allow(non_snake_case)]
    pub fn operator_call(&self, variant: &ConfigTableKey) -> usize {
        match &variant.0 {
            Variant2::V0(value) => {
                let hasher = DenseHashDefault::<alloc::string::String>::default();
                hasher.hash(value)
            }
            Variant2::V1(value) => {
                // f64 does not implement Hash in Rust.
                // Luau's DenseHashDefault for double typically hashes the bit representation.
                let hasher = DenseHashDefault::<u64>::default();
                hasher.hash(&value.to_bits())
            }
        }
    }
}

impl DenseHasher<ConfigTableKey> for VariantHashDefault {
    fn hash(&self, key: &ConfigTableKey) -> usize {
        self.operator_call(key)
    }
}
