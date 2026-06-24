use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;

use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::never_type::NeverType;
use crate::records::singleton_type::SingletonType;

use crate::functions::get_type_alt_j::get_type_id;

impl Normalizer {
    pub fn union_of_bools(&mut self, here: TypeId, there: TypeId) -> TypeId {
        self.consume_fuel();

        if !unsafe { get_type_id::<NeverType>(here).is_null() } {
            return there;
        }
        if !unsafe { get_type_id::<NeverType>(there).is_null() } {
            return here;
        }

        if let Some(hbool) = unsafe {
            get_type_id::<SingletonType>(here)
                .as_ref()
                .and_then(|s| s.variant.get_if::<BooleanSingleton>())
        } {
            if let Some(tbool) = unsafe {
                get_type_id::<SingletonType>(there)
                    .as_ref()
                    .and_then(|s| s.variant.get_if::<BooleanSingleton>())
            } {
                if hbool.value == tbool.value {
                    return here;
                }
            }
        }

        unsafe { (*self.builtin_types).booleanType }
    }
}
