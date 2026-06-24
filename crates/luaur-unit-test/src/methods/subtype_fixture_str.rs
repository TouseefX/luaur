use crate::records::subtype_fixture::SubtypeFixture;
use alloc::string::ToString;
use luaur_analysis::records::singleton_type::SingletonType;
use luaur_analysis::records::string_singleton::StringSingleton;
use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn str(&mut self, literal: &str) -> TypeId {
        self.arena.add_type(SingletonType {
            variant: SingletonVariant::V1(StringSingleton {
                value: literal.to_string(),
            }),
        })
    }
}
