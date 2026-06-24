use crate::functions::get_code_gen_context::get_code_gen_context;
use crate::functions::userdata_remapper_wrap::userdata_remapper_wrap;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::userdata_remapper_callback::UserdataRemapperCallback;
use core::ffi::c_void;

/// Sets the userdata remapper callback and context on the code generation context.
///
/// Mirrors `setUserdataRemapper` (CodeGen/src/CodeGenContext.cpp):
/// ```cpp
/// void setUserdataRemapper(lua_State* L, void* context, UserdataRemapperCallback cb) {
///     if (BaseCodeGenContext* codegenCtx = getCodeGenContext(L)) {
///         codegenCtx->userdataRemappingContext = context;
///         codegenCtx->userdataRemapper = cb;
///         L->global->ecb.gettypemapping = cb ? userdataRemapperWrap : nullptr;
///     }
/// }
/// ```
///
/// # Safety
///
/// - `L` must be a valid, non-null pointer to a `lua_State`.
/// - The global state (`L->global`) must be valid and initialized.
/// - The code generation context must be valid and properly initialized.
/// - The callback `cb` must be a valid function pointer.
#[inline]
pub fn set_userdata_remapper(
    L: *mut lua_State,
    context: *mut c_void,
    cb: UserdataRemapperCallback,
) {
    // get_code_gen_context handles the null checks (L / global / context) internally.
    let codegen_ctx = get_code_gen_context(L);
    if codegen_ctx.is_null() {
        return;
    }

    // SAFETY: codegen_ctx is non-null and points to a valid BaseCodeGenContext;
    // the caller guarantees L (and L->global) are valid per the contract above.
    unsafe {
        (*codegen_ctx).userdata_remapping_context = context;
        // C++ stores the function pointer directly (`userdataRemapper = cb`).
        // The Rust field is `Option<UserdataRemapperCallback>`, so wrap the value
        // itself rather than taking the address of the local `cb`.
        (*codegen_ctx).userdata_remapper = Some(cb);

        // C++: L->global->ecb.gettypemapping = cb ? userdataRemapperWrap : nullptr;
        // The translated `cb` parameter is a non-nullable function pointer, so a
        // remapper is always present here and the wrapper trampoline is installed.
        let global = (*L).global;
        if !global.is_null() {
            (*global).ecb.gettypemapping = Some(userdata_remapper_wrap);
        }
    }
}
