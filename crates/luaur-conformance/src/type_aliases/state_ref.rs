use core::ptr::NonNull;

/// A RAII wrapper around a `lua_State*` that automatically closes the state on drop.
/// Mirrors `std::unique_ptr<lua_State, void (*)(lua_State*)>` from C++.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StateRef(pub NonNull<luaur_vm::records::lua_state::lua_State>);

impl StateRef {
    /// Creates a new `StateRef` from a non-null `lua_State*` pointer.
    /// The caller is responsible for ensuring the pointer was allocated with `lua_newstate`.
    pub fn new(state: *mut luaur_vm::records::lua_state::lua_State) -> Option<Self> {
        NonNull::new(state).map(Self)
    }

    /// Returns the raw `lua_State*` pointer.
    pub fn as_ptr(&self) -> *mut luaur_vm::records::lua_state::lua_State {
        self.0.as_ptr()
    }
}

impl Drop for StateRef {
    fn drop(&mut self) {
        unsafe {
            luaur_vm::functions::lua_close::lua_close(self.as_ptr());
        }
    }
}
