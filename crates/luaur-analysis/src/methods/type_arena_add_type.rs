use crate::records::r#type::Type;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeArena {
    pub fn add_type<T>(&mut self, tv: T) -> crate::type_aliases::type_id::TypeId
    where
        T: Into<Type> + 'static,
    {
        let tv_into: Type = tv.into();

        if let TypeVariant::Union(union_type) = &tv_into.ty {
            LUAU_ASSERT!(union_type.options.len() >= 2);
        }

        if let TypeVariant::Singleton(singleton_type) = &tv_into.ty {
            if self.collect_singleton_stats {
                self.record_singleton_stats(singleton_type);
            }
        }

        self.add_tv(tv_into)
    }
}
