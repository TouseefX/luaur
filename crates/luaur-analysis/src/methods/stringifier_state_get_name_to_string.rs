//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:209:stringifier_state_get_name`
//! Source: `Analysis/src/ToString.cpp:209-227` (hand-ported)

use crate::functions::generate_name::generate_name;
use crate::records::stringifier_state::StringifierState;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

impl StringifierState {
    /// C++ `std::string getName(TypeId ty)`.
    pub fn get_name_type_id(&mut self, ty: TypeId) -> String {
        unsafe {
            let opts = &mut *self.opts;
            let s = opts.name_map.types.size();
            // std::string& n = opts.nameMap.types[ty]; (default-constructs)
            {
                let n = opts.name_map.types.get_or_insert(ty);
                if !n.is_empty() {
                    return n.clone();
                }
            }

            for count in 0..256usize {
                let candidate = generate_name(self.used_names.size() + count);
                if !self.used_names.contains(&candidate) {
                    self.used_names.insert(candidate.clone());
                    *opts.name_map.types.get_or_insert(ty) = candidate.clone();
                    return candidate;
                }
            }

            generate_name(s)
        }
    }
}
