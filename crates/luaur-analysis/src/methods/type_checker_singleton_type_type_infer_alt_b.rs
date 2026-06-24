use crate::records::r#type::Type;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

impl TypeChecker {
    pub fn singleton_type_string(&mut self, value: String) -> TypeId {
        let singleton = SingletonType::singleton_type(
            SingletonVariant::variant_t_enable_if_t_get_type_id_t(StringSingleton::new(value)),
        );
        let ty = Type::new(TypeVariant::Singleton(singleton));
        self.add_type(&ty)
    }
}
