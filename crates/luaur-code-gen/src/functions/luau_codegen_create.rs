use crate::type_aliases::lua_state::lua_State;

pub fn luau_codegen_create(_L: *mut lua_State) {
    // The Rust port does not link Luau's native C++ codegen context creation yet.
}
