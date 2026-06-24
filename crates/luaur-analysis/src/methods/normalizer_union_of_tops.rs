use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn union_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
        self.consume_fuel();

        if !unsafe { get_type_id::<NeverType>(here).is_null() }
            || !unsafe { get_type_id::<AnyType>(there).is_null() }
        {
            return there;
        }

        here
    }
}
