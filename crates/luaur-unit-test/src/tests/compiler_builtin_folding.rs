//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:9209:compiler_builtin_folding`
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
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> function lrotate (CodeGen/src/BitUtils.h)
//!   - calls -> function rrotate (CodeGen/src/BitUtils.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item compiler_builtin_folding

#[cfg(test)]
#[test]
fn compiler_builtin_folding() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"
return
    math.abs(-42),
    math.acos(1),
    math.asin(0),
    math.atan2(0, 1),
    math.atan(0),
    math.ceil(1.5),
    math.cosh(0),
    math.cos(0),
    math.deg(3.14159265358979323846),
    math.exp(0),
    math.floor(-1.5),
    math.fmod(7, 3),
    math.ldexp(0.5, 3),
    math.log10(100),
    math.log(1),
    math.log(4, 2),
    math.log(64, 4),
    math.max(1, 2, 3),
    math.min(1, 2, 3),
    math.pow(3, 3),
    math.floor(math.rad(180)),
    math.sinh(0),
    math.sin(0),
    math.sqrt(9),
    math.tanh(0),
    math.tan(0),
    bit32.arshift(-10, 1),
    bit32.arshift(10, 1),
    bit32.band(1, 3),
    bit32.bnot(-2),
    bit32.bor(1, 2),
    bit32.bxor(3, 7),
    bit32.btest(1, 3),
    bit32.extract(100, 1, 3),
    bit32.lrotate(100, -1),
    bit32.lshift(100, 1),
    bit32.replace(100, 5, 1, 3),
    bit32.rrotate(100, -1),
    bit32.rshift(100, 1),
    type(100),
    string.byte("a"),
    string.byte("abc", 2),
    string.len("abc"),
    typeof(true),
    math.clamp(-1, 0, 1),
    math.sign(77),
    math.round(7.6),
    bit32.extract(-1, 31),
    bit32.replace(100, 1, 0),
    math.log(100, 10),
    typeof(nil),
    type(vector.create(1, 0, 0)),
    (type("fin")),
    math.isnan(0/0),
    math.isnan(0),
    math.isinf(math.huge),
    math.isinf(-4),
    math.isfinite(42),
    math.isfinite(-math.huge)
"#,
        0,
        2,
        0,
    );
    let expected = r#"
LOADN R0 42
LOADN R1 0
LOADN R2 0
LOADN R3 0
LOADN R4 0
LOADN R5 2
LOADN R6 1
LOADN R7 1
LOADN R8 180
LOADN R9 1
LOADN R10 -2
LOADN R11 1
LOADN R12 4
LOADN R13 2
LOADN R14 0
LOADN R15 2
LOADN R16 3
LOADN R17 3
LOADN R18 1
LOADN R19 27
LOADN R20 3
LOADN R21 0
LOADN R22 0
LOADN R23 3
LOADN R24 0
LOADN R25 0
LOADK R26 K0 [4294967291]
LOADN R27 5
LOADN R28 1
LOADN R29 1
LOADN R30 3
LOADN R31 4
LOADB R32 1
LOADN R33 2
LOADN R34 50
LOADN R35 200
LOADN R36 106
LOADN R37 200
LOADN R38 50
LOADK R39 K1 ['number']
LOADN R40 97
LOADN R41 98
LOADN R42 3
LOADK R43 K2 ['boolean']
LOADN R44 0
LOADN R45 1
LOADN R46 8
LOADN R47 1
LOADN R48 101
LOADN R49 2
LOADK R50 K3 ['nil']
LOADK R51 K4 ['vector']
LOADK R52 K5 ['string']
LOADB R53 1
LOADB R54 0
LOADB R55 1
LOADB R56 0
LOADB R57 1
LOADB R58 0
RETURN R0 59
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
