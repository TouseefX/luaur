//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:37:to_string_options_to_string_options`
//! Source: `Analysis/include/Luau/ToString.h:37-60` (hand-ported)

use crate::records::to_string_name_map::ToStringNameMap;
use crate::records::to_string_options::ToStringOptions;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ToStringOptions {
    /// C++ `ToStringOptions(bool exhaustive = false)` with the in-class
    /// member initializers from ToString.h:43-59.
    pub fn to_string_options(exhaustive: bool) -> Self {
        ToStringOptions {
            exhaustive,
            use_line_breaks: false,
            function_type_arguments: false,
            hide_table_kind: false,
            hide_named_function_type_parameters: false,
            hide_function_self_argument: false,
            hide_table_alias_expansions: false,
            use_question_marks: true,
            ignore_synthetic_name: false,
            max_table_length: luaur_common::FInt::LuauTableTypeMaximumStringifierLength.get()
                as usize,
            max_type_length: luaur_common::FInt::LuauTypeMaximumStringifierLength.get() as usize,
            composite_types_single_line_limit: 5,
            name_map: ToStringNameMap {
                types: DenseHashMap::new(core::ptr::null()),
                type_packs: DenseHashMap::new(core::ptr::null()),
            },
            scope: None,
            named_function_override_arg_names: Vec::new(),
        }
    }
}

impl Default for ToStringOptions {
    /// C++ `ToStringOptions{}`.
    fn default() -> Self {
        Self::to_string_options(false)
    }
}
