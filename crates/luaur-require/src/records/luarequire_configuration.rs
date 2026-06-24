use crate::enums::luarequire_config_status::luarequire_ConfigStatus;
use crate::enums::luarequire_navigate_result::luarequire_NavigateResult;
use crate::enums::luarequire_write_result::luarequire_WriteResult;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct luarequire_Configuration {
    pub is_require_allowed: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            requirer_chunkname: *const core::ffi::c_char,
        ) -> bool,
    >,
    pub reset: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            requirer_chunkname: *const core::ffi::c_char,
        ) -> luarequire_NavigateResult,
    >,
    pub jump_to_alias: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            path: *const core::ffi::c_char,
        ) -> luarequire_NavigateResult,
    >,
    pub to_alias_override: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            alias_unprefixed: *const core::ffi::c_char,
        ) -> luarequire_NavigateResult,
    >,
    pub to_alias_fallback: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            alias_unprefixed: *const core::ffi::c_char,
        ) -> luarequire_NavigateResult,
    >,
    pub to_parent: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
        ) -> luarequire_NavigateResult,
    >,
    pub to_child: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            name: *const core::ffi::c_char,
        ) -> luarequire_NavigateResult,
    >,
    pub is_module_present: Option<
        unsafe extern "C" fn(l: *mut core::ffi::c_void, ctx: *mut core::ffi::c_void) -> bool,
    >,
    pub get_chunkname: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            buffer: *mut core::ffi::c_char,
            buffer_size: usize,
            size_out: *mut usize,
        ) -> luarequire_WriteResult,
    >,
    pub get_loadname: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            buffer: *mut core::ffi::c_char,
            buffer_size: usize,
            size_out: *mut usize,
        ) -> luarequire_WriteResult,
    >,
    pub get_cache_key: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            buffer: *mut core::ffi::c_char,
            buffer_size: usize,
            size_out: *mut usize,
        ) -> luarequire_WriteResult,
    >,
    pub get_config_status: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
        ) -> luarequire_ConfigStatus,
    >,
    pub get_alias: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            alias: *const core::ffi::c_char,
            buffer: *mut core::ffi::c_char,
            buffer_size: usize,
            size_out: *mut usize,
        ) -> luarequire_WriteResult,
    >,
    pub get_config: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            buffer: *mut core::ffi::c_char,
            buffer_size: usize,
            size_out: *mut usize,
        ) -> luarequire_WriteResult,
    >,
    pub get_luau_config_timeout: Option<
        unsafe extern "C" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
        ) -> core::ffi::c_int,
    >,
    // `load` may raise a Lua error (e.g. when the required module fails at
    // runtime). With the panic-based `luaD_throw`, that error unwinds out of the
    // callback, so the boundary must permit unwinding (`C-unwind`).
    pub load: Option<
        unsafe extern "C-unwind" fn(
            l: *mut core::ffi::c_void,
            ctx: *mut core::ffi::c_void,
            path: *const core::ffi::c_char,
            chunkname: *const core::ffi::c_char,
            loadname: *const core::ffi::c_char,
        ) -> core::ffi::c_int,
    >,
}
