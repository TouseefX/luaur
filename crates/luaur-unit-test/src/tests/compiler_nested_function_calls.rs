//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:3125:compiler_nested_function_calls`
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
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item compiler_nested_function_calls

#[cfg(test)]
#[test]
fn compiler_nested_function_calls() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("function clamp(t,a,b) return math.min(math.max(t,a),b) end");
    let expected = r#"
FASTCALL2 18 R0 R1 L0
MOVE R5 R0
MOVE R6 R1
GETIMPORT R4 2 [math.max]
CALL R4 2 1
L0: FASTCALL2 19 R4 R2 L1
MOVE R5 R2
GETIMPORT R3 4 [math.min]
CALL R3 2 -1
L1: RETURN R3 -1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
