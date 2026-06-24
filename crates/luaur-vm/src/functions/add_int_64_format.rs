use crate::macros::max_format::MAX_FORMAT;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[allow(non_snake_case)]
pub(crate) fn add_int_64_format(
    form: &mut [core::ffi::c_char],
    format_indicator: core::ffi::c_char,
    format_item_size: usize,
) {
    LUAU_ASSERT!((format_item_size + 3) <= MAX_FORMAT as usize);
    LUAU_ASSERT!(form[0] == '%' as core::ffi::c_char);
    LUAU_ASSERT!(form[format_item_size] != 0);
    LUAU_ASSERT!(form[format_item_size + 1] == 0);
    form[format_item_size + 0] = 'l' as core::ffi::c_char;
    form[format_item_size + 1] = 'l' as core::ffi::c_char;
    form[format_item_size + 2] = format_indicator;
    form[format_item_size + 3] = 0;
}
