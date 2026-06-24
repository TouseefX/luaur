use alloc::string::String;
use core::ffi::CStr;

use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
use luaur_compiler::functions::compile::compile;
use luaur_compiler::records::compile_options::CompileOptions;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::type_aliases::lua_state::lua_State;

pub fn load(l: *mut lua_State, source: &String) -> Option<String> {
    struct NoopEncoder;

    impl BytecodeEncoder for NoopEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
    }

    let options = CompileOptions::default();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let bytecode = compile(
        source,
        &options,
        &parse_options,
        &mut encoder as *mut dyn BytecodeEncoder,
    );

    unsafe {
        if luau_load(
            l,
            c"=config".as_ptr(),
            bytecode.as_ptr() as *const core::ffi::c_char,
            bytecode.len(),
            0,
        ) != 0
        {
            return Some(lua_string(l, -1));
        }
    }

    None
}

unsafe fn lua_string(l: *mut lua_State, index: i32) -> String {
    let ptr = lua_tolstring(l, index, core::ptr::null_mut());
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}
