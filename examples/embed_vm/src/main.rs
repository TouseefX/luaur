//! Embed the VM directly: compile, load onto a fresh thread, run, and read the
//! returned value. Use this lower-level (C-style) API when you need more control
//! than `luaur::eval` — custom globals, inspecting results, multiple chunks, etc.
//!
//!     cargo run -p luaur-example-embed-vm

use luaur::vm::functions::lua_gettop::lua_gettop;
use luaur::vm::functions::lua_l_newstate::lua_l_newstate;
use luaur::vm::functions::lua_l_openlibs::lua_l_openlibs;
use luaur::vm::functions::lua_newthread::lua_newthread;
use luaur::vm::functions::lua_resume::lua_resume;
use luaur::vm::functions::lua_tonumberx::lua_tonumberx;
use luaur::vm::functions::luau_load::luau_load;

fn main() {
    let bytecode = luaur::compile("return 6 * 7").expect("compile failed");

    // v11+ bytecode needs the default Luau flags (matches the CLI).
    luaur::common::set_all_flags(true);

    unsafe {
        let l = lua_l_newstate();
        assert!(!l.is_null(), "could not create Lua state");
        lua_l_openlibs(l);

        // Run on a fresh thread, like the reference CLI's runCode.
        let t = lua_newthread(l);
        assert!(!t.is_null(), "could not create thread");

        let rc = luau_load(
            t,
            c"=embed".as_ptr(),
            bytecode.as_ptr() as *const core::ffi::c_char,
            bytecode.len(),
            0,
        );
        assert_eq!(rc, 0, "luau_load failed (rc={rc})");

        let status = lua_resume(t, core::ptr::null_mut(), 0);
        assert_eq!(status, 0, "script raised an error (status={status})");

        // The chunk's return values are left on the thread's stack.
        let n = lua_gettop(t);
        println!("script returned {n} value(s):");
        for i in 1..=n {
            let mut is_num: core::ffi::c_int = 0;
            let v = lua_tonumberx(t, i, &mut is_num);
            if is_num != 0 {
                println!("  [{i}] = {v}");
            }
        }
    }
}
