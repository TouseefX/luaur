//! Test fixture: faithful port of `compileFunction` + `luauLibraryConstantLookup`
//! (tests/Compiler.test.cpp).
use core::ffi::{c_char, CStr};

use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
use luaur_compiler::functions::set_compile_constant_boolean::set_compile_constant_boolean;
use luaur_compiler::functions::set_compile_constant_nil::set_compile_constant_nil;
use luaur_compiler::functions::set_compile_constant_number::set_compile_constant_number;
use luaur_compiler::functions::set_compile_constant_string::set_compile_constant_string;
use luaur_compiler::functions::set_compile_constant_vector::set_compile_constant_vector;
use luaur_compiler::records::compile_options::CompileOptions;
use luaur_compiler::type_aliases::compile_constant::CompileConstant;

// Port of the test's `luauLibraryConstantLookup` callback. The compiler passes
// `&mut Constant as *mut CompileConstant` (CompileConstant = *mut c_void), so the
// incoming `constant` pointer's bits ARE the `*mut Constant`; `as CompileConstant`
// recovers it for the `set_compile_constant_*` helpers.
unsafe extern "C" fn luau_library_constant_lookup(
    library: *const c_char,
    member: *const c_char,
    constant: *mut CompileConstant,
) {
    let cc = constant as CompileConstant;
    let lib = CStr::from_ptr(library);
    let mem = CStr::from_ptr(member);

    if lib == c"vector" {
        if mem == c"zero" {
            return set_compile_constant_vector(cc, 0.0, 0.0, 0.0, 0.0);
        }
        if mem == c"one" {
            return set_compile_constant_vector(cc, 1.0, 1.0, 1.0, 0.0);
        }
    }

    if lib == c"Vector3" {
        if mem == c"one" {
            return set_compile_constant_vector(cc, 1.0, 1.0, 1.0, 0.0);
        }
        if mem == c"xAxis" {
            return set_compile_constant_vector(cc, 1.0, 0.0, 0.0, 0.0);
        }
    }

    if lib == c"test" {
        if mem == c"some_nil" {
            return set_compile_constant_nil(cc);
        }
        if mem == c"some_boolean" {
            return set_compile_constant_boolean(cc, true);
        }
        if mem == c"some_number" {
            return set_compile_constant_number(cc, 4.75);
        }
        if mem == c"some_string" {
            return set_compile_constant_string(cc, c"test".as_ptr(), 4);
        }
    }
}

pub fn compile_function(
    source: &str,
    id: u32,
    optimization_level: i32,
    type_info_level: i32,
) -> alloc::string::String {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let mut options = CompileOptions::default();
    options.optimization_level = optimization_level;
    options.type_info_level = type_info_level;
    options.vector_lib = c"Vector3".as_ptr();
    options.vector_ctor = c"new".as_ptr();

    let libraries_with_constants: [*const c_char; 4] = [
        c"vector".as_ptr(),
        c"Vector3".as_ptr(),
        c"test".as_ptr(),
        core::ptr::null(),
    ];
    options.libraries_with_known_members = libraries_with_constants.as_ptr();
    options.library_member_constant_cb = Some(luau_library_constant_lookup);

    let source = alloc::string::String::from(source);
    let parse_options = ParseOptions::default();
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    bcb.dump_function(id)
}
