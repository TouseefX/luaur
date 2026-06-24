//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:178:stringifier_state_stringifier_state`
//! Source: `Analysis/src/ToString.cpp:178-190` (hand-ported)

use crate::records::set::Set;
use crate::records::stringifier_state::StringifierState;
use crate::records::to_string_options::ToStringOptions;
use crate::records::to_string_result::ToStringResult;
use alloc::string::String;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl StringifierState {
    /// C++ `StringifierState(ToStringOptions& opts, ToStringResult& result)`.
    /// The C++ reference members are raw pointers in the record; callers keep
    /// `opts`/`result` alive for the state's lifetime (as in C++).
    pub fn stringifier_state_stringifier_state(
        opts: *mut ToStringOptions,
        result: *mut ToStringResult,
    ) -> Self {
        unsafe {
            let o = &*opts;
            let mut state = StringifierState {
                opts,
                result,
                cycle_names: DenseHashMap::new(core::ptr::null()),
                cycle_tp_names: DenseHashMap::new(core::ptr::null()),
                seen: Set::new(core::ptr::null_mut()),
                // `$$$` is the usedNames tombstone: not a valid name syntactically
                // and short for string comparison reasons.
                used_names: DenseHashSet::new(String::from("$$$")),
                indentation: 0,
                exhaustive: o.exhaustive,
                ignore_synthetic_name: o.ignore_synthetic_name,
                previous_name_index: 0,
            };

            for (_k, v) in o.name_map.types.iter() {
                state.used_names.insert(v.clone());
            }
            for (_k, v) in o.name_map.type_packs.iter() {
                state.used_names.insert(v.clone());
            }

            state
        }
    }
}
