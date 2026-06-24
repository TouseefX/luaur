use crate::records::singleton_type::SingletonType;
use crate::type_aliases::singleton_variant::SingletonVariant;

impl SingletonType {
    pub fn singleton_type_mut(variant: SingletonVariant) -> Self {
        Self { variant }
    }
}
