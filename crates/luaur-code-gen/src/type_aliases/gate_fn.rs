use crate::records::native_context::NativeContext;
use crate::type_aliases::lua_state::lua_State;

#[allow(non_camel_case_types)]
pub type GateFn = Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        proto: *mut core::ffi::c_void,
        arg2: usize,
        context: *mut NativeContext,
    ) -> core::ffi::c_int,
>;
