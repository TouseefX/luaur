//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:431:compiler_fake_import_call`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function compileFunction (tests/Compiler.test.cpp)
//!   - translates_to -> rust_item compiler_fake_import_call

#[cfg(test)]
#[test]
fn compiler_fake_import_call() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let source =
        "math = {} function math.max() return 0 end function test() return math.max(1, 2) end";

    let actual = compile_function(source, 1, 1, 0);
    let expected = r#"
GETGLOBAL R0 K0 ['math']
GETTABLEKS R0 R0 K1 ['max']
LOADN R1 1
LOADN R2 2
CALL R0 2 -1
RETURN R0 -1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
