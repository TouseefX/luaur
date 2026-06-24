use crate::records::serialized_generic::SerializedGeneric;

impl<T> SerializedGeneric<T> {
    pub fn serialized_generic_bool_string_t(
        &mut self,
        is_named: bool,
        name: alloc::string::String,
        r#type: T,
    ) {
        self.is_named = is_named;
        self.name = name;
        self.r#type = r#type;
    }
}
