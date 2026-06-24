use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeMap;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct NormalizedStringType {
    /// When false, this type represents a union of singleton string types.
    /// eg "a" | "b" | "c"
    ///
    /// When true, this type represents string intersected with negated string
    /// singleton types.
    /// eg string & ~"a" & ~"b" & ...
    pub(crate) isCofinite: bool,

    pub(crate) singletons: BTreeMap<String, TypeId>,
}

impl Default for NormalizedStringType {
    fn default() -> Self {
        Self {
            isCofinite: false,
            singletons: BTreeMap::new(),
        }
    }
}

#[allow(non_upper_case_globals)]
impl NormalizedStringType {
    pub const never: NormalizedStringType = NormalizedStringType {
        isCofinite: false,
        singletons: BTreeMap::new(),
    };
}
