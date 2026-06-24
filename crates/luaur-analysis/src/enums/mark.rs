#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Mark {
    None,
    Temporary,
    Permanent,
}

impl Mark {
    pub const None: Self = Self::None;
    pub const Temporary: Self = Self::Temporary;
    pub const Permanent: Self = Self::Permanent;
}

impl Default for Mark {
    fn default() -> Self {
        Self::None
    }
}

// Required so `Mark` can be a `DenseHashMap` value type in `Frontend::parseGraph`
// (C++: `DenseHashMap<SourceNode*, Mark> seen(nullptr)`). The map default-inserts
// `Mark{}` for absent keys, which is the zero-initialized first enumerator `None`.
impl luaur_common::records::dense_hash_table::DenseDefault for Mark {
    fn dense_default() -> Self {
        Self::None
    }
}
