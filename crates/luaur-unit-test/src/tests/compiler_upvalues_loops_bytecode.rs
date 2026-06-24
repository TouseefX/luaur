//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:3141:compiler_upvalues_loops_bytecode`
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
//!   - calls -> function compileFunction (tests/Compiler.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item compiler_upvalues_loops_bytecode

#[cfg(test)]
#[test]
fn compiler_upvalues_loops_bytecode() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let actual = compile_function(
        r#"
function test()
    for i=1,10 do
        i = i
        foo(function() return i end)
        if bar then
            break
        end
    end
    return 0
end
"#,
        1,
        1,
        0,
    );
    let expected = r#"
LOADN R2 1
LOADN R0 10
LOADN R1 1
FORNPREP R0 L2
L0: MOVE R3 R2
GETIMPORT R4 1 [foo]
NEWCLOSURE R5 P0
CAPTURE REF R3
CALLFB R4 1 0 [0]
GETIMPORT R4 3 [bar]
JUMPIFNOT R4 L1
CLOSEUPVALS R3
JUMP L2
L1: CLOSEUPVALS R3
FORNLOOP R0 L0
L2: LOADN R0 0
RETURN R0 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function(
        r#"
function test()
    for i in ipairs(data) do
        i = i
        foo(function() return i end)
        if bar then
            break
        end
    end
    return 0
end
"#,
        1,
        1,
        0,
    );
    let expected = r#"
GETIMPORT R0 1 [ipairs]
GETIMPORT R1 3 [data]
CALLFB R0 1 3 [0]
FORGPREP_INEXT R0 L2
L0: GETIMPORT R5 5 [foo]
NEWCLOSURE R6 P0
CAPTURE REF R3
CALLFB R5 1 0 [1]
GETIMPORT R5 7 [bar]
JUMPIFNOT R5 L1
CLOSEUPVALS R3
JUMP L3
L1: CLOSEUPVALS R3
L2: FORGLOOP R0 L0 1 [inext]
L3: LOADN R0 0
RETURN R0 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function(
        r#"
function test()
    local i = 0
    while i < 5 do
        local j
        j = i
        foo(function() return j end)
        i = i + 1
        if bar then
            break
        end
    end
    return 0
end
"#,
        1,
        1,
        0,
    );
    let expected = r#"
LOADN R0 0
L0: LOADN R1 5
JUMPIFNOTLT R0 R1 L2
LOADNIL R1
MOVE R1 R0
GETIMPORT R2 1 [foo]
NEWCLOSURE R3 P0
CAPTURE REF R1
CALLFB R2 1 0 [0]
ADDK R0 R0 K2 [1]
GETIMPORT R2 4 [bar]
JUMPIFNOT R2 L1
CLOSEUPVALS R1
JUMP L2
L1: CLOSEUPVALS R1
JUMPBACK L0
L2: LOADN R1 0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function(
        r#"
function test()
    local i = 0
    repeat
        local j
        j = i
        foo(function() return j end)
        i = i + 1
        if bar then
            break
        end
    until i < 5
    return 0
end
"#,
        1,
        1,
        0,
    );
    let expected = r#"
LOADN R0 0
L0: LOADNIL R1
MOVE R1 R0
GETIMPORT R2 1 [foo]
NEWCLOSURE R3 P0
CAPTURE REF R1
CALLFB R2 1 0 [0]
ADDK R0 R0 K2 [1]
GETIMPORT R2 4 [bar]
JUMPIFNOT R2 L1
CLOSEUPVALS R1
JUMP L3
L1: LOADN R2 5
JUMPIFLT R0 R2 L2
CLOSEUPVALS R1
JUMPBACK L0
L2: CLOSEUPVALS R1
L3: LOADN R1 0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
