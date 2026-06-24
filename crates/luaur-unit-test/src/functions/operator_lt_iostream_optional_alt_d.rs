use alloc::string::String;

use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::dense_hash_table::{DenseEq, DenseHasher};

pub fn operator_lt_ostream_luau_dense_hash_set_k_h_e<K, H, E>(set: &DenseHashSet<K, H, E>) -> String
where
    K: core::fmt::Display + Clone,
    H: Default + DenseHasher<K>,
    E: Default + DenseEq<K>,
{
    let mut result = String::from("{ ");
    let mut first = true;

    for element in set.iter() {
        if first {
            first = false;
        } else {
            result.push_str(", ");
        }

        result.push_str(&format!("{}", element));
    }

    result.push_str(" }");
    result
}
