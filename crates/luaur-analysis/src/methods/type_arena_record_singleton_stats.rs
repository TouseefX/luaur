use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_arena::TypeArena;
use luaur_common::records::variant::Variant2;

impl TypeArena {
    pub fn record_singleton_stats(&mut self, singleton: &SingletonType) {
        match &singleton.variant {
            Variant2::V0(bool_singleton) => {
                self.bool_singletons_minted += 1;
            }
            Variant2::V1(str_singleton) => {
                self.str_singletons_minted += 1;
                if !str_singleton.value.is_empty() {
                    self.unique_str_singletons_minted
                        .insert(Some(str_singleton.value.clone()));
                }
            }
        }
    }
}
