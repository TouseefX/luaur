//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:10498:compiler_if_then_else_and_or`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item compiler_if_then_else_and_or

#[cfg(test)]
#[test]
fn compiler_if_then_else_and_or() {
    use crate::functions::compile_function_0::compile_function_0;

    // if v then v else k can be optimized to ORK
    let actual = compile_function_0(
        r#"
local x = ...
return if x then x else 0
"#,
    );
    let expected = r#"
GETVARARGS R0 1
ORK R1 R0 K0 [0]
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // if v then v else l can be optimized to OR
    let actual = compile_function_0(
        r#"
local x, y = ...
return if x then x else y
"#,
    );
    let expected = r#"
GETVARARGS R0 2
OR R2 R0 R1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // this also works in presence of type casts
    let actual = compile_function_0(
        r#"
local x, y = ...
return if x then x :: number else 0
"#,
    );
    let expected = r#"
GETVARARGS R0 2
ORK R2 R0 K0 [0]
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // if v then k else v can be optimized to ANDK
    let actual = compile_function_0(
        r#"
local x = ...
return if x then 0 else x
"#,
    );
    let expected = r#"
GETVARARGS R0 1
ANDK R1 R0 K0 [0]
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // if v then l else v can be optimized to AND
    let actual = compile_function_0(
        r#"
local x, y = ...
return if x then y else x
"#,
    );
    let expected = r#"
GETVARARGS R0 2
AND R2 R0 R1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // this also works in presence of type casts
    let actual = compile_function_0(
        r#"
local x, y = ...
return if x then y else x :: number
"#,
    );
    let expected = r#"
GETVARARGS R0 2
AND R2 R0 R1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // all of the above work when the target is a temporary register, which is safe because the value is only mutated once
    let actual = compile_function_0(
        r#"
local x, y = ...
x = if x then x else y
x = if x then y else x
"#,
    );
    let expected = r#"
GETVARARGS R0 2
OR R0 R0 R1
AND R0 R0 R1
RETURN R0 0
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // note that we can't do this transformation if the expression has possible side effects
    let actual = compile_function_0(
        r#"
local x = ...
return if x.data then x.data else 0
"#,
    );
    let expected = r#"
GETVARARGS R0 1
GETTABLEKS R2 R0 K0 ['data']
JUMPIFNOT R2 L0
GETTABLEKS R1 R0 K0 ['data']
RETURN R1 1
L0: LOADN R1 0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
