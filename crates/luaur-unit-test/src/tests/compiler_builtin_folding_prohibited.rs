//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:9343:compiler_builtin_folding_prohibited`
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
//!   - calls -> function compileFunction (tests/Compiler.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item compiler_builtin_folding_prohibited

#[cfg(test)]
#[test]
fn compiler_builtin_folding_prohibited() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"
return
    math.abs(),
    math.max(1, true),
    string.byte("abc", 42),
    bit32.rshift(10, 42),
    bit32.extract(1, 2, "3"),
    bit32.bor(1, true),
    bit32.band(1, true),
    bit32.bxor(1, true),
    bit32.btest(1, true),
    math.min(1, true),
    typeof(vector.create(1, 0, 0))
"#,
        0,
        2,
        0,
    );
    let expected = r#"
FASTCALL 2 L0
GETIMPORT R0 2 [math.abs]
CALL R0 0 1
L0: LOADN R2 1
FASTCALL2K 18 R2 K3 L1 [true]
LOADK R3 K3 [true]
GETIMPORT R1 5 [math.max]
CALL R1 2 1
L1: LOADK R3 K6 ['abc']
FASTCALL2K 41 R3 K7 L2 [42]
LOADK R4 K7 [42]
GETIMPORT R2 10 [string.byte]
CALL R2 2 1
L2: LOADN R4 10
FASTCALL2K 39 R4 K7 L3 [42]
LOADK R5 K7 [42]
GETIMPORT R3 13 [bit32.rshift]
CALL R3 2 1
L3: LOADN R5 1
LOADN R6 2
LOADK R7 K14 ['3']
FASTCALL 34 L4
GETIMPORT R4 16 [bit32.extract]
CALL R4 3 1
L4: LOADN R6 1
FASTCALL2K 31 R6 K3 L5 [true]
LOADK R7 K3 [true]
GETIMPORT R5 18 [bit32.bor]
CALL R5 2 1
L5: LOADN R7 1
FASTCALL2K 29 R7 K3 L6 [true]
LOADK R8 K3 [true]
GETIMPORT R6 20 [bit32.band]
CALL R6 2 1
L6: LOADN R8 1
FASTCALL2K 32 R8 K3 L7 [true]
LOADK R9 K3 [true]
GETIMPORT R7 22 [bit32.bxor]
CALL R7 2 1
L7: LOADN R9 1
FASTCALL2K 33 R9 K3 L8 [true]
LOADK R10 K3 [true]
GETIMPORT R8 24 [bit32.btest]
CALL R8 2 1
L8: LOADN R10 1
FASTCALL2K 19 R10 K3 L9 [true]
LOADK R11 K3 [true]
GETIMPORT R9 26 [math.min]
CALL R9 2 1
L9: LOADK R11 K27 [1, 0, 0]
FASTCALL1 44 R11 L10
GETIMPORT R10 29 [typeof]
CALL R10 1 1
L10: RETURN R0 11
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
