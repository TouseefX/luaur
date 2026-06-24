//! The [`Thread`] handle and [`ThreadStatus`]. Mirrors `mlua::Thread` /
//! `mlua::ThreadStatus`.
//!
//! A thread is a Luau coroutine. It is created from a [`Function`] via
//! [`Lua::create_thread`] (or surfaces from `coroutine.create(...)` evaluated
//! in Lua) and driven with [`Thread::resume`].
//!
//! ## Implementation
//!
//! The thread is a first-class Lua value, so the handle holds a registry
//! reference (like every other handle) keeping the coroutine alive. We also
//! cache the raw `*mut lua_State` of the coroutine for the resume/xmove dance.
//!
//! `resume` mirrors mlua: push the args onto the *parent* state, `lua_xmove`
//! them to the coroutine, `lua_resume(co, parent, nargs)`, then `lua_xmove` the
//! results back and convert them. Status is derived from `lua_status` +
//! `lua_costatus`, matching mlua's `Resumable`/`Running`/`Normal`/`Finished`/
//! `Error` mapping.

use std::rc::Rc;

use crate::error::{Error, Result};
use crate::ffi::*;
use crate::function::Function;
use crate::multi::MultiValue;
use crate::state::{Lua, LuaRef};
use crate::traits::{FromLuaMulti, IntoLua, IntoLuaMulti};

/// Status of a Lua thread (coroutine). Mirrors `mlua::ThreadStatus`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ThreadStatus {
    /// The thread was just created or is suspended (yielded) and can be resumed.
    Resumable,
    /// The thread is currently running.
    Running,
    /// The thread is active but not running (it resumed another thread).
    Normal,
    /// The thread has finished executing.
    Finished,
    /// The thread raised a Lua error during execution.
    Error,
}

/// A handle to a Lua thread (coroutine). Mirrors `mlua::Thread`.
#[derive(Clone)]
pub struct Thread {
    pub(crate) reference: Rc<LuaRef>,
    /// The raw coroutine state pointer (cached from the referenced value).
    pub(crate) thread_state: *mut lua_State,
}

impl Thread {
    /// Build a [`Thread`] from a registry ref to a thread value. Caches the
    /// coroutine's raw state via `lua_tothread`.
    pub(crate) fn from_ref(reference: LuaRef) -> Thread {
        let state = reference.state();
        let thread_state = unsafe {
            reference.push();
            let ts = lua_tothread(state, -1);
            lua_pop(state, 1);
            ts
        };
        Thread {
            reference: Rc::new(reference),
            thread_state,
        }
    }

    pub(crate) unsafe fn push_to_stack(&self) {
        self.reference.push();
    }

    /// The owning [`Lua`].
    pub fn lua(&self) -> Lua {
        self.reference.lua()
    }

    /// The raw coroutine state pointer. Mirrors `mlua::Thread::state`.
    pub fn state(&self) -> *mut lua_State {
        self.thread_state
    }

    /// Resume the coroutine, passing `args` and converting its yielded/returned
    /// values to `R`. Mirrors `mlua::Thread::resume`.
    ///
    /// Returns [`Error::CoroutineUnresumable`] if the thread has finished,
    /// errored, or is otherwise not resumable.
    pub fn resume<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> Result<R> {
        let lua = self.lua();
        // Convert the args first so a failing `IntoLua` (e.g. a bad argument)
        // surfaces *before* we touch any Lua stack — matching mlua.
        let args: MultiValue = args.into_lua_multi(&lua)?;

        if !matches!(self.status(), ThreadStatus::Resumable) {
            return Err(Error::CoroutineUnresumable);
        }

        let parent = lua.state();
        let co = self.thread_state;
        unsafe {
            let nargs = args.len() as c_int;
            if lua_checkstack(co, nargs.saturating_add(2)) == 0 {
                return Err(Error::RuntimeError(
                    "stack overflow: too many arguments to coroutine resume".to_string(),
                ));
            }
            // Push args onto the parent, then move them to the coroutine.
            for v in args.iter() {
                lua.push_value(v)?;
            }
            if nargs > 0 {
                lua_xmove(parent, co, nargs);
            }

            self.resume_inner::<R>(&lua, nargs)
        }
    }

    /// Resume the coroutine, immediately raising `error` inside it.
    /// Mirrors `mlua::Thread::resume_error` (a Luau extension).
    pub fn resume_error<R: FromLuaMulti>(&self, error: impl IntoLua) -> Result<R> {
        let lua = self.lua();
        let err_value = error.into_lua(&lua)?;

        if !matches!(self.status(), ThreadStatus::Resumable) {
            return Err(Error::CoroutineUnresumable);
        }

        let parent = lua.state();
        let co = self.thread_state;
        unsafe {
            if lua_checkstack(co, 2) == 0 {
                return Err(Error::RuntimeError("stack overflow".to_string()));
            }
            lua.push_value(&err_value)?;
            lua_xmove(parent, co, 1);
            // lua_resumeerror does the resume-with-error and returns the status.
            let status = lua_resumeerror(co, parent);
            self.finish_resume::<R>(&lua, status)
        }
    }

    /// Run `lua_resume` and collect/convert the results. Expects `nargs` already
    /// moved onto the coroutine stack.
    unsafe fn resume_inner<R: FromLuaMulti>(&self, lua: &Lua, nargs: c_int) -> Result<R> {
        let parent = lua.state();
        let co = self.thread_state;
        let status = unsafe { lua_resume(co, parent, nargs) };
        unsafe { self.finish_resume::<R>(lua, status) }
    }

    /// Common tail of `resume`/`resume_error`: inspect the status, move results
    /// back to the parent, and convert.
    unsafe fn finish_resume<R: FromLuaMulti>(&self, lua: &Lua, status: c_int) -> Result<R> {
        let parent = lua.state();
        let co = self.thread_state;
        unsafe {
            if status != status::OK && status != status::YIELD {
                // Error: the coroutine left the error object on its own stack.
                let nres = lua_gettop(co);
                if nres > 0 {
                    lua_xmove(co, parent, nres);
                }
                let err = lua.pop_error(status);
                // Clear any extra values the coroutine left on the parent.
                return Err(err);
            }
            // Success/yield: the produced values sit on the coroutine stack.
            let nres = lua_gettop(co);
            if lua_checkstack(parent, nres.saturating_add(1)) == 0 {
                return Err(Error::RuntimeError("stack overflow".to_string()));
            }
            let base = lua_gettop(parent);
            if nres > 0 {
                lua_xmove(co, parent, nres);
            }
            let mut results = MultiValue::with_capacity(nres.max(0) as usize);
            for i in 0..nres {
                results.push_back(lua.value_from_stack(base + 1 + i)?);
            }
            lua_settop(parent, base);
            R::from_lua_multi(results, lua)
        }
    }

    /// The thread's status. Mirrors `mlua::Thread::status`.
    pub fn status(&self) -> ThreadStatus {
        let lua = self.lua();
        let parent = lua.state();
        let co = self.thread_state;
        // A thread whose state is the currently-running state is "Running".
        if co == parent {
            return ThreadStatus::Running;
        }
        unsafe {
            let cos = lua_costatus(parent, co);
            match cos {
                costatus::SUSPENDED => ThreadStatus::Resumable,
                costatus::RUNNING => ThreadStatus::Running,
                costatus::NORMAL => ThreadStatus::Normal,
                costatus::FINISHED => ThreadStatus::Finished,
                costatus::ERROR => ThreadStatus::Error,
                _ => {
                    // Fall back to lua_status for any unexpected code.
                    let s = lua_status(co);
                    if s == status::YIELD {
                        ThreadStatus::Resumable
                    } else if s == status::OK {
                        // New (function on stack) vs finished (empty stack).
                        if lua_gettop(co) > 0 {
                            ThreadStatus::Resumable
                        } else {
                            ThreadStatus::Finished
                        }
                    } else {
                        ThreadStatus::Error
                    }
                }
            }
        }
    }

    /// Whether the thread can be resumed. Mirrors `mlua::Thread::is_resumable`.
    pub fn is_resumable(&self) -> bool {
        self.status() == ThreadStatus::Resumable
    }

    /// Whether the thread is currently running. Mirrors `mlua::Thread::is_running`.
    pub fn is_running(&self) -> bool {
        self.status() == ThreadStatus::Running
    }

    /// Whether the thread is active but not running. Mirrors
    /// `mlua::Thread::is_normal`.
    pub fn is_normal(&self) -> bool {
        self.status() == ThreadStatus::Normal
    }

    /// Whether the thread has finished executing. Mirrors
    /// `mlua::Thread::is_finished`.
    pub fn is_finished(&self) -> bool {
        self.status() == ThreadStatus::Finished
    }

    /// Whether the thread raised an error. Mirrors `mlua::Thread::is_error`.
    pub fn is_error(&self) -> bool {
        self.status() == ThreadStatus::Error
    }

    /// Reset the thread to a fresh state and install `func` as its body.
    /// Mirrors `mlua::Thread::reset` (Luau semantics: any non-running thread can
    /// be reset).
    pub fn reset(&self, func: Function) -> Result<()> {
        let status = self.status();
        match status {
            ThreadStatus::Running => {
                return Err(Error::runtime("cannot reset a running thread"));
            }
            ThreadStatus::Normal => {
                return Err(Error::runtime("cannot reset a normal thread"));
            }
            _ => {}
        }
        let lua = self.lua();
        let parent = lua.state();
        let co = self.thread_state;
        unsafe {
            lua_resetthread(co);
            // Push the new body function onto the coroutine stack.
            func.push_to_stack();
            lua_xmove(parent, co, 1);
        }
        Ok(())
    }

    /// A raw pointer identifying this thread. Mirrors `mlua::Thread::to_pointer`.
    pub fn to_pointer(&self) -> *const c_void {
        let state = self.reference.state();
        unsafe {
            self.reference.push();
            let p = lua_topointer(state, -1);
            lua_pop(state, 1);
            p
        }
    }
}

impl std::fmt::Debug for Thread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thread")
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.to_pointer() == other.to_pointer()
    }
}

impl IntoLua for Thread {
    fn into_lua(self, _lua: &Lua) -> Result<crate::value::Value> {
        Ok(crate::value::Value::Thread(self))
    }
}

impl IntoLua for &Thread {
    fn into_lua(self, _lua: &Lua) -> Result<crate::value::Value> {
        Ok(crate::value::Value::Thread(self.clone()))
    }
}

impl FromLua for Thread {
    fn from_lua(value: crate::value::Value, _lua: &Lua) -> Result<Self> {
        match value {
            crate::value::Value::Thread(t) => Ok(t),
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "Thread".to_string(),
                message: None,
            }),
        }
    }
}

use crate::traits::FromLua;

impl Lua {
    /// Create a new coroutine from a [`Function`]. Mirrors
    /// `mlua::Lua::create_thread`.
    pub fn create_thread(&self, func: Function) -> Result<Thread> {
        let state = self.state();
        unsafe {
            // Create a new thread; it is pushed on the parent stack.
            let co = lua_newthread(state);
            if co.is_null() {
                return Err(Error::runtime("luaur-rt: failed to create thread"));
            }
            // Take a ref to the thread value (still on the parent stack top).
            let thread = Thread::from_ref(self.pop_ref());
            // Move the body function onto the coroutine's stack so the first
            // resume invokes it.
            func.push_to_stack(); // pushes onto parent stack
            lua_xmove(state, co, 1);
            Ok(thread)
        }
    }

    /// The currently-running thread. Mirrors `mlua::Lua::current_thread`.
    ///
    /// Inside a Rust callback this is the coroutine (or main thread) that
    /// invoked it.
    pub fn current_thread(&self) -> Thread {
        let state = self.state();
        unsafe {
            lua_pushthread(state);
            Thread::from_ref(self.pop_ref())
        }
    }
}
