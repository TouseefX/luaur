//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:408:compiler_import_call_redirect_local_polyfill`
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
//!   - translates_to -> rust_item compiler_import_call_redirect_local_polyfill

#[cfg(test)]
#[test]
fn compiler_import_call_redirect_local_polyfill() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0(
        r#"
local math = math or require("math-polyfill")
return math.max(1, 2)
"#,
    );
    let expected = r#"
GETIMPORT R0 1 [math]
JUMPIF R0 L0
GETIMPORT R0 3 [require]
LOADK R1 K4 ['math-polyfill']
CALL R0 1 1
L0: LOADN R2 1
FASTCALL2K 18 R2 K5 L1 [2]
LOADK R3 K5 [2]
GETTABLEKS R1 R0 K6 ['max']
CALL R1 2 -1
L1: RETURN R1 -1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
