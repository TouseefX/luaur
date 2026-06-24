use crate::records::builtin_types::BuiltinTypes;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
    pub fn normalized_type_not_null_builtin_types(&mut self, builtin_types: *mut BuiltinTypes) {
        self.builtin_types = builtin_types;
        let never_type = unsafe { (*builtin_types).neverType };

        self.tops = never_type;
        self.booleans = never_type;
        self.errors = never_type;
        self.nils = never_type;
        self.numbers = never_type;
        self.integers = never_type;
        self.strings = NormalizedStringType::never;
        self.threads = never_type;
        self.buffers = never_type;
    }
}
