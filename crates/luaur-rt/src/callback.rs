//! The Rust-closure-as-Lua-function trampoline.
//!
//! ## Design (see also the crate-level docs)
//!
//! A user-supplied Rust closure is type-erased to
//! [`BoxedCallback`] = `Box<dyn Fn(&Lua, MultiValue) -> Result<MultiValue>>`.
//! That `Box` is stored inside a Lua userdata created with
//! [`lua_newuserdatadtor`] — the destructor reconstitutes and drops the `Box`,
//! so the closure's captured environment is freed exactly when the GC collects
//! the function. The userdata is then captured as **upvalue 1** of a single
//! C trampoline function ([`trampoline`]) pushed via [`lua_pushcclosurek`]
//! with `nup = 1`.
//!
//! When Lua calls the function:
//!  1. The trampoline fetches upvalue 1 with [`lua_upvalueindex`] and recovers
//!     `&BoxedCallback` from the userdata pointer.
//!  2. It pops all on-stack arguments into a [`MultiValue`].
//!  3. It runs the closure **inside [`catch_unwind`]** — so a `panic!` in user
//!     code can never become a nested panic while we are about to call
//!     [`lua_error`].
//!  4. On success it pushes the results and returns the count.
//!  5. On a returned `Err`, or a caught panic, it pushes a message string and
//!     calls [`lua_error`]. `lua_error` raises the VM's normal longjmp-style
//!     error (a `panic_any(lua_exception)`), which unwinds this trampoline
//!     frame up to the VM's protected-call boundary — the VM's own mechanism.
//!
//! Because the user panic is caught *before* `lua_error` is called, there is
//! never a double-unwind, and a genuine Rust panic in user code surfaces as an
//! ordinary catchable Lua error, not a process abort.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::error::{Error, Result};
use crate::ffi::*;
use crate::function::Function;
use crate::multi::MultiValue;
use crate::state::Lua;

/// The type-erased boxed callback stored in the trampoline's upvalue userdata.
pub(crate) type BoxedCallback = Box<dyn Fn(&Lua, MultiValue) -> Result<MultiValue>>;

/// The destructor installed on the callback userdata: reconstruct the `Box`
/// inside the userdata storage and drop it (calling `Drop` on captures).
///
/// `lua_newuserdatadtor` stores the data inline; `lua_touserdata` returns a
/// pointer to that storage, which is exactly where we wrote the
/// `BoxedCallback`. We drop it in place.
unsafe extern "C" fn callback_dtor(ptr: *mut c_void) {
    if !ptr.is_null() {
        let bc = ptr as *mut BoxedCallback;
        unsafe { core::ptr::drop_in_place(bc) };
    }
}

/// The one C trampoline shared by every `create_function` closure.
unsafe fn trampoline(state: *mut lua_State) -> c_int {
    unsafe {
        // 1. Recover the boxed callback from upvalue 1.
        let ud = lua_touserdata(state, lua_upvalueindex(1));
        if ud.is_null() {
            // Should be impossible; fail loudly but safely via lua_error.
            return raise_lua_error(state, "luaur-rt: missing callback upvalue");
        }
        let callback = &*(ud as *const BoxedCallback);

        // 2. Build a borrowed Lua handle for the calling thread (must NOT close it).
        let lua = Lua::from_borrowed(state);

        // 3. Pull the arguments off the stack into a MultiValue. They occupy
        //    stack indices 1..=nargs.
        let nargs = lua_gettop(state);
        let args = match collect_args(&lua, nargs) {
            Ok(a) => a,
            Err(e) => return raise_lua_error(state, &e.to_string()),
        };

        // 4. Run the user closure inside catch_unwind so a user `panic!` never
        //    becomes a nested panic when we then call lua_error.
        let outcome: std::thread::Result<Result<MultiValue>> =
            catch_unwind(AssertUnwindSafe(|| callback(&lua, args)));

        match outcome {
            Ok(Ok(results)) => {
                // 5a. Push every result and return its count.
                let n = results.len() as c_int;
                for v in results.iter() {
                    if let Err(e) = lua.push_value(v) {
                        return raise_lua_error(state, &e.to_string());
                    }
                }
                n
            }
            Ok(Err(err)) => {
                // 5b. The closure returned Err -> raise it as a Lua error.
                raise_lua_error(state, &err.to_string())
            }
            Err(panic_payload) => {
                // 5c. The closure panicked -> turn it into a catchable Lua error.
                let msg = panic_message(&panic_payload);
                raise_lua_error(state, &format!("rust panic: {msg}"))
            }
        }
    }
}

/// The actual `lua_CFunction` pointer (an `Option<unsafe fn(...)>`).
fn trampoline_ptr() -> lua_CFunction {
    Some(trampoline)
}

/// Collect stack arguments `1..=nargs` into a [`MultiValue`].
unsafe fn collect_args(lua: &Lua, nargs: c_int) -> Result<MultiValue> {
    let mut m = MultiValue::with_capacity(nargs.max(0) as usize);
    for i in 1..=nargs {
        m.push_back(lua.value_from_stack(i)?);
    }
    Ok(m)
}

/// Push `msg` as the error object and invoke [`lua_error`]. Never returns
/// normally (it unwinds), but is typed `-> c_int` so call sites read naturally.
unsafe fn raise_lua_error(state: *mut lua_State, msg: &str) -> c_int {
    unsafe {
        lua_pushlstring(state, msg.as_ptr() as *const c_char, msg.len());
        lua_error(state) // diverges (`-> !`)
    }
}

/// Best-effort extraction of a panic payload's message.
fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

/// Build a [`Function`] from a type-erased boxed callback. Used by
/// [`Lua::create_function`] and the userdata method machinery.
pub(crate) fn create_callback_function(lua: &Lua, callback: BoxedCallback) -> Result<Function> {
    let state = lua.state();
    unsafe {
        // Allocate userdata sized for a BoxedCallback, with our dtor.
        let storage = lua_newuserdatadtor(
            state,
            core::mem::size_of::<BoxedCallback>(),
            Some(callback_dtor),
        );
        if storage.is_null() {
            return Err(Error::runtime("luaur-rt: failed to allocate callback userdata"));
        }
        // Move the box into the userdata storage (do NOT run its drop here).
        core::ptr::write(storage as *mut BoxedCallback, callback);

        // The userdata is now on top of the stack; capture it as upvalue 1 of
        // the trampoline closure.
        lua_pushcclosurek(
            state,
            trampoline_ptr(),
            c"luaur-rt-callback".as_ptr(),
            1, // nup: consumes the userdata above as upvalue 1
            None,
        );
        // The closure is now on top; take a registry ref.
        Ok(Function::from_ref(lua.pop_ref()))
    }
}
