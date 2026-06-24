//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:377:compiler_import_call`
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
//!   - calls -> function compileFunction0 (tests/Compiler.test.cpp)
//!   - translates_to -> rust_item compiler_import_call

#[cfg(test)]
#[test]
fn compiler_import_call() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("return math.max(1, 2)");
    let expected = r#"
LOADN R1 1
FASTCALL2K 18 R1 K0 L0 [2]
LOADK R2 K0 [2]
GETIMPORT R0 3 [math.max]
CALL R0 2 -1
L0: RETURN R0 -1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
