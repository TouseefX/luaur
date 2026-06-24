use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::widen::Widen;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Widen {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        LUAU_ASSERT!(self.is_dirty_type_id(ty));

        let stv = unsafe { crate::functions::get_type_alt_j::get_type_id::<SingletonType>(ty) };
        LUAU_ASSERT!(!stv.is_null());

        let stv_ref = unsafe { stv.as_ref().unwrap() };

        if stv_ref.variant.get_if::<StringSingleton>().is_some() {
            unsafe { (*self.builtin_types).stringType }
        } else {
            // If this assert trips, it's likely we now have number singletons.
            LUAU_ASSERT!(stv_ref.variant.get_if::<BooleanSingleton>().is_some());
            unsafe { (*self.builtin_types).booleanType }
        }
    }
}
