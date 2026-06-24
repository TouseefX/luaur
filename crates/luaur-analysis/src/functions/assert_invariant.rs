use crate::functions::are_normalized_extern_types::are_normalized_extern_types;
use crate::functions::are_normalized_functions::are_normalized_functions;
use crate::functions::are_normalized_tables::are_normalized_tables;
use crate::functions::is_normalized_boolean::is_normalized_boolean;
use crate::functions::is_normalized_buffer::is_normalized_buffer;
use crate::functions::is_normalized_error::is_normalized_error;
use crate::functions::is_normalized_integer::is_normalized_integer;
use crate::functions::is_normalized_nil::is_normalized_nil;
use crate::functions::is_normalized_number::is_normalized_number;
use crate::functions::is_normalized_string::is_normalized_string;
use crate::functions::is_normalized_thread::is_normalized_thread;
use crate::functions::is_normalized_top::is_normalized_top;
use crate::functions::is_normalized_tyvar::is_normalized_tyvar;
use crate::records::normalized_type::NormalizedType;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub(crate) fn assert_invariant(norm: &NormalizedType) {
    if !luaur_common::FFlag::DebugLuauCheckNormalizeInvariant.get() {
        return;
    }

    LUAU_ASSERT!(is_normalized_top(norm.tops));
    LUAU_ASSERT!(is_normalized_boolean(norm.booleans));
    LUAU_ASSERT!(are_normalized_extern_types(&norm.extern_types));
    LUAU_ASSERT!(is_normalized_error(norm.errors));
    LUAU_ASSERT!(is_normalized_nil(norm.nils));
    LUAU_ASSERT!(is_normalized_number(norm.numbers));
    if luaur_common::FFlag::LuauIntegerType2.get() {
        LUAU_ASSERT!(is_normalized_integer(norm.integers));
    }
    LUAU_ASSERT!(is_normalized_string(&norm.strings));
    LUAU_ASSERT!(is_normalized_thread(norm.threads));
    LUAU_ASSERT!(is_normalized_buffer(norm.buffers));
    LUAU_ASSERT!(are_normalized_functions(&norm.functions));
    LUAU_ASSERT!(are_normalized_tables(&norm.tables));
    LUAU_ASSERT!(is_normalized_tyvar(&norm.tyvars));
    for child in norm.tyvars.values() {
        assert_invariant(child);
    }
}
