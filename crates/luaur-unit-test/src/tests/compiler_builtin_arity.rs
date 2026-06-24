//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:10010:compiler_builtin_arity`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item compiler_builtin_arity

#[cfg(test)]
#[test]
fn compiler_builtin_arity() {
    use crate::functions::compile_function::compile_function;

    // by default we can't assume that we know parameter/result count for builtins as they can be overridden at runtime
    let actual = compile_function(
        r#"
return math.abs(unknown())
"#,
        0,
        1,
        0,
    );
    let expected = r#"
GETIMPORT R1 1 [unknown]
CALL R1 0 -1
FASTCALL 2 L0
GETIMPORT R0 4 [math.abs]
CALL R0 -1 -1
L0: RETURN R0 -1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // however, when using optimization level 2, we assume compile time knowledge about builtin behavior even if we can't deoptimize that with fenv
    // in the test case below, this allows us to synthesize a more efficient FASTCALL1 (and use a fixed-return call to unknown)
    let actual = compile_function(
        r#"
return math.abs(unknown())
"#,
        0,
        2,
        0,
    );
    let expected = r#"
GETIMPORT R1 1 [unknown]
CALL R1 0 1
FASTCALL1 2 R1 L0
GETIMPORT R0 4 [math.abs]
CALL R0 1 1
L0: RETURN R0 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // some builtins are variadic, and as such they can't use fixed-length fastcall variants
    let actual = compile_function(
        r#"
return math.max(0, unknown())
"#,
        0,
        2,
        0,
    );
    let expected = r#"
LOADN R1 0
GETIMPORT R2 1 [unknown]
CALL R2 0 -1
FASTCALL 18 L0
GETIMPORT R0 4 [math.max]
CALL R0 -1 1
L0: RETURN R0 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // some builtins are not variadic but don't have a fixed number of arguments; we currently don't optimize this although we might start to in the
    // future
    let actual = compile_function(
        r#"
return bit32.extract(0, 1, unknown())
"#,
        0,
        2,
        0,
    );
    let expected = r#"
LOADN R1 0
LOADN R2 1
GETIMPORT R3 1 [unknown]
CALL R3 0 -1
FASTCALL 34 L0
GETIMPORT R0 4 [bit32.extract]
CALL R0 -1 1
L0: RETURN R0 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // some builtins are not variadic and have a fixed number of arguments but are not none-safe, meaning that we can't replace calls that may
    // return none with calls that will return nil
    let actual = compile_function(
        r#"
return type(unknown())
"#,
        0,
        2,
        0,
    );
    let expected = r#"
GETIMPORT R1 1 [unknown]
CALL R1 0 -1
FASTCALL 40 L0
GETIMPORT R0 3 [type]
CALL R0 -1 1
L0: RETURN R0 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // importantly, this optimization also helps us get around the multret inlining restriction for builtin wrappers
    let actual = compile_function(
        r#"
local function new()
    return setmetatable({}, MT)
end

return new()
"#,
        1,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['new']
NEWTABLE R2 0 0
GETIMPORT R3 2 [MT]
FASTCALL2 61 R2 R3 L0
GETIMPORT R1 4 [setmetatable]
CALL R1 2 1
L0: RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // note that the results of this optimization are benign in fixed-arg contexts which dampens the effect of fenv substitutions on correctness in
    // practice
    let actual = compile_function(
        r#"
local x = ...
local y, z = type(x)
return type(y, z)
"#,
        0,
        2,
        0,
    );
    let expected = r#"
GETVARARGS R0 1
FASTCALL1 40 R0 L0
MOVE R2 R0
GETIMPORT R1 1 [type]
CALL R1 1 2
L0: FASTCALL2 40 R1 R2 L1
MOVE R4 R1
MOVE R5 R2
GETIMPORT R3 1 [type]
CALL R3 2 1
L1: RETURN R3 1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
