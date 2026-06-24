use alloc::string::String;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct TypeRehydrationOptions {
    pub(crate) banned_names: DenseHashSet<String>,
    pub(crate) expand_extern_type_props: bool,
}

impl Default for TypeRehydrationOptions {
    fn default() -> Self {
        Self {
            banned_names: DenseHashSet::new(String::new()),
            expand_extern_type_props: false,
        }
    }
}

#[allow(non_snake_case)]
impl TypeRehydrationOptions {
    pub fn bannedNames(&self) -> &DenseHashSet<String> {
        &self.banned_names
    }

    pub fn expandExternTypeProps(&self) -> bool {
        self.expand_extern_type_props
    }
}
