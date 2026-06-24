use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::normalized_type::NormalizedType;
use crate::records::singleton_type::SingletonType;
use luaur_common::FFlag;

impl NormalizedType {
    pub fn is_falsy(&self) -> bool {
        let mut has_a_false = false;
        let singleton_ptr = unsafe { get_type_id::<SingletonType>(self.booleans) };
        if !singleton_ptr.is_null() {
            let boolean_ptr = get_singleton_type::<BooleanSingleton>(singleton_ptr);
            if !boolean_ptr.is_null() {
                has_a_false = unsafe { !(*boolean_ptr).value };
            }
        }

        if FFlag::LuauIntegerType2.get() {
            (has_a_false || self.has_nils())
                && !self.has_tops()
                && !self.has_extern_types()
                && !self.has_errors()
                && !self.has_numbers()
                && !self.has_strings()
                && !self.has_threads()
                && !self.has_buffers()
                && !self.has_tables()
                && !self.has_functions()
                && !self.has_tyvars()
                && !self.has_integers()
        } else {
            (has_a_false || self.has_nils())
                && !self.has_tops()
                && !self.has_extern_types()
                && !self.has_errors()
                && !self.has_numbers()
                && !self.has_strings()
                && !self.has_threads()
                && !self.has_buffers()
                && !self.has_tables()
                && !self.has_functions()
                && !self.has_tyvars()
        }
    }
}
