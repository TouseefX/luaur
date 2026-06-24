//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:3575:compiler_debug_line_info_fast_call`
//! Source: `tests/Compiler.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Compiler.test.cpp
//! - source_includes:
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Compiler.test.cpp
//! - outgoing:
//!   - type_ref -> record BytecodeBuilder (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - calls -> method BytecodeBuilder::setDumpFlags (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> method BytecodeBuilder::dumpFunction (Bytecode/src/BytecodeBuilder.cpp)
//!   - translates_to -> rust_item compiler_debug_line_info_fast_call

#[cfg(test)]
#[test]
fn compiler_debug_line_info_fast_call() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from(
        r#"
local Foo, Bar = ...

return
    math.max(
        Foo,
        Bar)
"#,
    );
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n2: GETVARARGS R0 2\n5: FASTCALL2 18 R0 R1 L0\n5: MOVE R3 R0\n5: MOVE R4 R1\n5: GETIMPORT R2 2 [math.max]\n5: CALL R2 2 -1\n5: L0: RETURN R2 -1\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
