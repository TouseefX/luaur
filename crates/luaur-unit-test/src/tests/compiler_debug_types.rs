#[cfg(test)]
#[test]
fn compiler_debug_types() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::FFlag::LuauEmitCallFeedback;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let _emit_call_fb = ScopedFastFlag::new(&LuauEmitCallFeedback, true);

    let source = r#"
local up: number = 2

function foo(e: vector, f: mat3, g: sequence)
    local h = e * e

    for i=1,3 do
        print(i)
    end

    print(e * f)
    print(g)
    print(h)

    up += a
    return a
end
"#;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_TYPES);
    bcb.set_dump_source(source);

    let mut options = luaur_compiler::records::compile_options::CompileOptions::default();
    options.vector_ctor = c"vector".as_ptr();
    options.vector_type = c"vector".as_ptr();
    options.type_info_level = 1;

    let k_userdata_compile_types: [*const core::ffi::c_char; 4] = [
        c"vec2".as_ptr(),
        c"color".as_ptr(),
        c"mat3".as_ptr(),
        core::ptr::null(),
    ];
    options.userdata_types = k_userdata_compile_types.as_ptr();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source.to_string(),
        &options,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = r#"
R0: vector [argument]
R1: mat3 [argument]
R2: userdata [argument]
U0: number
R6: number from 1 to 10
R3: vector from 0 to 34
MUL R3 R0 R0
LOADN R6 1
LOADN R4 3
LOADN R5 1
FORNPREP R4 L1
L0: GETIMPORT R7 1 [print]
MOVE R8 R6
CALLFB R7 1 0 [0]
FORNLOOP R4 L0
L1: GETIMPORT R4 1 [print]
MUL R5 R0 R1
CALLFB R4 1 0 [1]
GETIMPORT R4 1 [print]
MOVE R5 R2
CALLFB R4 1 0 [2]
GETIMPORT R4 1 [print]
MOVE R5 R3
CALLFB R4 1 0 [3]
GETUPVAL R4 0
GETIMPORT R5 3 [a]
ADD R4 R4 R5
SETUPVAL R4 0
GETIMPORT R4 3 [a]
RETURN R4 1
"#;

    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
