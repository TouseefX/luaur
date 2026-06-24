use crate::records::serialized_generic::SerializedGeneric;

impl<T> SerializedGeneric<T> {
    pub fn serialized_generic_string(&mut self, name: alloc::string::String) {
        self.serialized_generic_bool_string_t(false, name, unsafe { core::mem::zeroed() });
    }
}
