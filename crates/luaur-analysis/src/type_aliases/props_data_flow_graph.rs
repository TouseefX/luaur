use crate::records::def::Def;
use alloc::collections::BTreeMap;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type Props = DenseHashMap<*const Def, BTreeMap<alloc::string::String, *const Def>>;
