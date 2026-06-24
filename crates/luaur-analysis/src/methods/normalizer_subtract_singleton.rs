use crate::functions::get_type_alt_j::get_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Normalizer {
    pub fn subtract_singleton(&mut self, here: &mut NormalizedType, ty: TypeId) {
        self.consume_fuel();

        let stv = unsafe { get_type_id::<SingletonType>(ty) };
        LUAU_ASSERT!(!stv.is_null());

        let stv = unsafe { &*stv };

        if let Some(ss) = stv.variant.get_if::<StringSingleton>() {
            if here.strings.isCofinite {
                here.strings.singletons.insert(ss.value.clone(), ty);
            } else {
                let it = here.strings.singletons.get_mut(&ss.value);
                if let Some(_) = it {
                    here.strings.singletons.remove(&ss.value);
                }
            }
        } else if let Some(bs) = stv.variant.get_if::<BooleanSingleton>() {
            if !unsafe { get_type_id::<NeverType>(here.booleans).is_null() } {
                // Nothing
            } else if !unsafe { get_type_id::<PrimitiveType>(here.booleans).is_null() } {
                let prim = unsafe { &*get_type_id::<PrimitiveType>(here.booleans) };
                if prim.r#type == PrimitiveType::Boolean {
                    here.booleans = if bs.value {
                        unsafe { (*self.builtin_types).falseType }
                    } else {
                        unsafe { (*self.builtin_types).trueType }
                    };
                }
            } else if let Some(here_singleton) = unsafe {
                get_type_id::<SingletonType>(here.booleans)
                    .as_ref()
                    .and_then(|s| s.variant.get_if::<BooleanSingleton>())
            } {
                // Crucial subtlety: ty (and thus bs) are the value that is being
                // negated out. We therefore reduce to never when the values match,
                // rather than when they differ.
                if bs.value == here_singleton.value {
                    here.booleans = unsafe { (*self.builtin_types).neverType };
                }
            } else {
                LUAU_ASSERT!(false);
            }
        } else {
            LUAU_ASSERT!(false);
        }
    }
}
