//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:8131:compiler_inline_non_const_initializers`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> macro upvalue (VM/src/lobject.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item compiler_inline_non_const_initializers

#[cfg(test)]
#[test]
fn compiler_inline_non_const_initializers() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let actual = compile_function(
        r#"
local function caller(f)
    f(1)
end

local function callback(n)
    print(n + 5)
end

caller(callback)
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['caller']
DUPCLOSURE R1 K1 ['callback']
GETIMPORT R2 3 [print]
LOADN R3 6
CALL R2 1 0
RETURN R0 0
"#;
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function(
        r#"
local x, y, z = ...
local function test(a, b, c, comp)
    return comp(a, b) and comp(b, c)
end

local function greater(a, b)
    return a > b
end

test(x, y, z, greater)
"#,
        2,
        2,
        0,
    );
    let expected = r#"
GETVARARGS R0 3
DUPCLOSURE R3 K0 ['test']
DUPCLOSURE R4 K1 ['greater']
JUMPIFLT R1 R0 L0
LOADB R5 0 +1
L0: LOADB R5 1
L1: JUMPIFNOT R5 L3
JUMPIFLT R2 R1 L2
LOADB R5 0 +1
L2: LOADB R5 1
L3: RETURN R0 0
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // inlined when passed as a temporary
    let actual = compile_function(
        r#"
local x, y, z = ...
local function test(a, b, c, comp)
    return comp(a, b) and comp(b, c)
end

test(x, y, z, function(a, b) return a > b end)
"#,
        2,
        2,
        0,
    );
    let expected = r#"
GETVARARGS R0 3
DUPCLOSURE R3 K0 ['test']
DUPCLOSURE R4 K1 []
JUMPIFLT R1 R0 L0
LOADB R5 0 +1
L0: LOADB R5 1
L1: JUMPIFNOT R5 L3
JUMPIFLT R2 R1 L2
LOADB R5 0 +1
L2: LOADB R5 1
L3: RETURN R0 0
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // inlined passed as an upvalue
    let actual = compile_function(
        r#"
local function test(a, b, c, comp)
    return comp(a, b) and comp(b, c)
end

local function greater(a, b)
    return a > b
end

local function bar(x, y, z)
    return test(x, y, z, greater)
end
"#,
        2,
        2,
        0,
    );
    let expected = r#"
GETUPVAL R4 0
JUMPIFLT R1 R0 L0
LOADB R3 0 +1
L0: LOADB R3 1
L1: JUMPIFNOT R3 L3
JUMPIFLT R2 R1 L2
LOADB R3 0 +1
L2: LOADB R3 1
L3: RETURN R3 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // not inlined when the upvalue is mutable
    let actual = compile_function(
        r#"
local function test(a, b, c, comp)
    return comp(a, b) and comp(b, c)
end

local function greater(a, b)
    return a > b
end

local function bar(x, y, z)
    return test(x, y, z, greater)
end

greater = function(a, b) return a < b end
"#,
        2,
        2,
        0,
    );
    let expected = r#"
GETUPVAL R4 0
MOVE R5 R4
MOVE R6 R0
MOVE R7 R1
CALLFB R5 2 1 [0]
MOVE R3 R5
JUMPIFNOT R3 L0
MOVE R5 R4
MOVE R6 R1
MOVE R7 R2
CALLFB R5 2 1 [1]
MOVE R3 R5
L0: RETURN R3 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // not inlined when argument itself is mutable
    let actual = compile_function(
        r#"
local x, y, z, debug = ...
local function test(a, b, c, comp)
    if debug then comp = function(a, b) return a >= b end end

    return comp(a, b) and comp(b, c)
end

test(x, y, z, function(a, b) return a > b end)
"#,
        3,
        2,
        0,
    );
    let expected = r#"
GETVARARGS R0 4
DUPCLOSURE R4 K0 ['test']
CAPTURE VAL R3
DUPCLOSURE R5 K1 []
JUMPIFNOT R3 L0
DUPCLOSURE R5 K2 []
L0: MOVE R6 R5
MOVE R7 R0
MOVE R8 R1
CALL R6 2 1
JUMPIFNOT R6 L1
MOVE R6 R5
MOVE R7 R1
MOVE R8 R2
CALL R6 2 1
L1: RETURN R0 0
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // inline builtins
    let actual = compile_function(
        r#"
local x, y, z = ...
local function test(a, b, c, d, op)
    return op(a, b) * op(c, d)
end

local min = math.min

local r1 = test(x, y, 2, 4, math.max)
local r2 = test(x, y, 2, 4, min)
local r3 = test(x, y, 2, 4, z)

return r1, r2, r3
"#,
        1,
        2,
        0,
    );
    let expected = r#"
GETVARARGS R0 3
DUPCLOSURE R3 K0 ['test']
GETIMPORT R4 3 [math.min]
GETIMPORT R6 5 [math.max]
FASTCALL2 18 R0 R1 L0
MOVE R8 R0
MOVE R9 R1
MOVE R7 R6
CALL R7 2 1
L0: MULK R5 R7 K6 [4]
FASTCALL2 19 R0 R1 L1
MOVE R8 R0
MOVE R9 R1
MOVE R7 R4
CALL R7 2 1
L1: MULK R6 R7 K7 [2]
MOVE R8 R2
MOVE R9 R0
MOVE R10 R1
CALL R8 2 1
MOVE R9 R2
LOADN R10 2
LOADN R11 4
CALL R9 2 1
MUL R7 R8 R9
RETURN R5 3
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
