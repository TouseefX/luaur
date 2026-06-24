use crate::type_aliases::singleton_variant::SingletonVariant;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingletonType {
    pub variant: SingletonVariant,
}

unsafe impl Send for SingletonType {}
unsafe impl Sync for SingletonType {}
