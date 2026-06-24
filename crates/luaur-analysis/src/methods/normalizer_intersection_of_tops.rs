use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn intersection_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
        self.consume_fuel();

        let here_is_never = !unsafe { get_type_id::<NeverType>(here).is_null() };
        let there_is_never = !unsafe { get_type_id::<NeverType>(there).is_null() };
        let here_is_any = !unsafe { get_type_id::<AnyType>(here).is_null() };
        let there_is_any = !unsafe { get_type_id::<AnyType>(there).is_null() };

        if here_is_never || there_is_never {
            return unsafe { (*self.builtin_types).neverType };
        }

        if here_is_any || there_is_any {
            return unsafe { (*self.builtin_types).anyType };
        }

        unsafe { (*self.builtin_types).unknownType }
    }
}
