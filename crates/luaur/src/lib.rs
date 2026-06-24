//! # luaur
//!
//! A faithful Rust translation of [Luau](https://github.com/luau-lang/luau) —
//! Roblox's typed Lua. This umbrella crate re-exports the individual layers
//! (lexer/parser/AST, bytecode, compiler, register VM, type checker, config and
//! require resolution) and provides two thin convenience helpers, [`compile`]
//! and [`eval`], for the common "compile a string / run a string" cases.
//!
//! For finer-grained control depend on the sub-crates directly; they are all
//! re-exported here as modules.
//!
//! ```
//! luaur::eval("assert(1 + 1 == 2)").unwrap();
//! let bytecode = luaur::compile("return 2 + 2").unwrap();
//! assert!(!bytecode.is_empty());
//! ```

// Re-export the sub-crates as modules so `luaur::vm::...` etc. work from one dep.
pub use luaur_analysis as analysis;
pub use luaur_ast as ast;
pub use luaur_bytecode as bytecode;
pub use luaur_common as common;
pub use luaur_compiler as compiler;
pub use luaur_config as config;
pub use luaur_require as require;
pub use luaur_vm as vm;

/// Common entry points, re-exported for convenience.
pub mod prelude {
    pub use crate::{compile, eval};
    pub use luaur_ast::records::parse_options::ParseOptions;
    pub use luaur_compiler::records::compile_options::CompileOptions;
}

use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
use luaur_compiler::functions::compile::compile as compiler_compile;
use luaur_compiler::records::compile_options::CompileOptions;

/// A no-op bytecode encoder — the compiler requires an encoder, and the default
/// (no encryption / no transform) one simply leaves the words untouched, exactly
/// like Luau's own `BytecodeEncoder` base class.
struct NoopEncoder;

impl BytecodeEncoder for NoopEncoder {
    fn encode(&mut self, _data: &mut [u32]) {}
}

/// Compile Luau `source` to bytecode using default compile/parse options.
///
/// On success the raw bytecode blob is returned. On a parse or compile error the
/// compiler emits an "error blob" (a leading `\0` marker byte followed by the
/// human-readable message); we detect that marker and surface the message as the
/// `Err` variant instead.
pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let options = CompileOptions::default();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let owned = source.to_string();

    let blob = compiler_compile(
        &owned,
        &options,
        &parse_options,
        &mut encoder as *mut dyn BytecodeEncoder,
    );

    let bytes = blob.into_bytes();
    // A leading 0 byte is the compiler's error marker (valid bytecode starts with
    // the non-zero LBC_VERSION_TARGET).
    if bytes.first() == Some(&0u8) {
        let message = String::from_utf8_lossy(&bytes[1..]).into_owned();
        return Err(message);
    }
    Ok(bytes)
}

/// Compile, load and run `source` on a fresh Luau VM, mirroring the reference
/// `luau` CLI (`luau_run` driver): a fresh state with the standard library open,
/// the chunk loaded into a new thread, and `lua_resume` to execute it.
///
/// Returns `Ok(())` if the script ran to completion, or `Err(message)` carrying
/// the Lua error string (the same text the CLI would print) on a compile, load
/// or runtime error.
pub fn eval(source: &str) -> Result<(), String> {
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_newthread::lua_newthread;
    use luaur_vm::functions::lua_resume::lua_resume;
    use luaur_vm::functions::lua_tolstring::lua_tolstring;
    use luaur_vm::functions::luau_load::luau_load;

    let bytecode = compile(source)?;

    // v11+ bytecode needs the default Luau flags enabled (matches the CLI's
    // setLuauFlagsDefault(true)).
    luaur_common::set_all_flags(true);

    unsafe {
        let l = lua_l_newstate();
        if l.is_null() {
            return Err("lua_l_newstate returned null".to_string());
        }
        lua_l_openlibs(l);

        // Run on a fresh thread, like CLI/src/Repl.cpp's runCode.
        let t = lua_newthread(l);
        if t.is_null() {
            return Err("lua_newthread returned null".to_string());
        }

        let rc = luau_load(
            t,
            c"=eval".as_ptr(),
            bytecode.as_ptr() as *const core::ffi::c_char,
            bytecode.len(),
            0,
        );
        if rc != 0 {
            return Err(format!("luau_load failed: rc={rc}"));
        }

        let status = lua_resume(t, core::ptr::null_mut(), 0);
        if status != 0 {
            // The error object is on top of T's stack; recover its text.
            let mut len = 0usize;
            let s = lua_tolstring(t, -1, &mut len);
            let msg = if s.is_null() {
                "<non-string error>".to_string()
            } else {
                let bytes = core::slice::from_raw_parts(s as *const u8, len);
                String::from_utf8_lossy(bytes).into_owned()
            };
            return Err(msg);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{compile, eval};

    #[test]
    fn compile_returns_nonempty_bytecode() {
        let bytecode = compile("return 1 + 1").expect("compile should succeed");
        assert!(!bytecode.is_empty(), "bytecode must be non-empty");
    }

    #[test]
    fn eval_runs_passing_assertion() {
        eval("assert(1 + 1 == 2)").expect("eval should succeed");
    }

    #[test]
    fn eval_reports_runtime_error() {
        let err = eval("error('boom')").expect_err("eval should fail");
        assert!(err.contains("boom"), "error message should mention boom: {err}");
    }
}
