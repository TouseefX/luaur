//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:3946:compiler_cost_model_remarks`
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
//!   - calls -> function compileWithRemarks (tests/Compiler.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> function writef32 (CodeGen/src/ByteUtils.h)
//!   - translates_to -> rust_item compiler_cost_model_remarks

#[cfg(test)]
#[test]
fn compiler_cost_model_remarks() {
    use crate::functions::compile_with_remarks::compile_with_remarks;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    assert_eq!(
        compile_with_remarks(
            r#"
local a, b = ...

local function foo(x)
    return(math.abs(x))
end

return foo(a) + foo(assert(b))
"#
        ),
        r#"
local a, b = ...

local function foo(x)
    -- remark: builtin math.abs/1
    return(math.abs(x))
end

-- remark: builtin assert/1
-- remark: inlining succeeded (cost 2, profit 2.50x, depth 0)
return foo(a) + foo(assert(b))
"#
    );

    assert_eq!(
        compile_with_remarks(
            r#"
local value = true

local function foo()
    return value
end

return foo()
"#
        ),
        r#"
local value = true

local function foo()
    return value
end

-- remark: inlining succeeded (cost 0, profit 3.00x, depth 0)
return foo()
"#
    );

    assert_eq!(
        compile_with_remarks(
            r#"
local value = true

local function foo()
    return not value
end

return foo()
"#
        ),
        r#"
local value = true

local function foo()
    return not value
end

-- remark: inlining succeeded (cost 0, profit 3.00x, depth 0)
return foo()
"#
    );

    assert_eq!(
        compile_with_remarks(
            r#"
local function foo()
    local s = 0
    for i = 1, 100 do s += i end
    return s
end

return foo()
"#
        ),
        r#"
local function foo()
    local s = 0
    -- remark: loop unroll failed: too many iterations (100)
    for i = 1, 100 do s += i end
    return s
end

-- remark: inlining failed: too expensive (cost 127, profit 1.02x)
return foo()
"#
    );

    assert_eq!(
        compile_with_remarks(
            r#"
local function foo()
    local s = 0
    for i = 1, 4 * 25 do s += i end
    return s
end

return foo()
"#
        ),
        r#"
local function foo()
    local s = 0
    -- remark: loop unroll failed: too many iterations (100)
    for i = 1, 4 * 25 do s += i end
    return s
end

-- remark: inlining failed: too expensive (cost 127, profit 1.02x)
return foo()
"#
    );

    assert_eq!(
        compile_with_remarks(
            r#"
local x = ...
local function test(a)
    while a < 0 do
        a += 1
    end
    for i=10,1,-1 do
        a += 1
    end
    for i in pairs({}) do
        a += 1
        if a % 2 == 0 then continue end
    end
    repeat
        a += 1
        if a % 2 == 0 then break end
    until a > 10
    return a
end
local a = test(x)
local b = test(2)
"#
        ),
        r#"
local x = ...
local function test(a)
    while a < 0 do
        a += 1
    end
    -- remark: loop unroll succeeded (iterations 10, cost 10, profit 2.00x)
    for i=10,1,-1 do
        a += 1
    end
    -- remark: allocation: table hash 0
    for i in pairs({}) do
        a += 1
        if a % 2 == 0 then continue end
    end
    repeat
        a += 1
        if a % 2 == 0 then break end
    until a > 10
    return a
end
-- remark: inlining failed: too expensive (cost 76, profit 1.03x)
local a = test(x)
-- remark: inlining failed: too expensive (cost 73, profit 1.08x)
local b = test(2)
"#
    );

    let _fastcall3 = ScopedFastFlag::new(&FFlag::LuauCompileFastcall3CostModel, true);

    assert_eq!(
        compile_with_remarks(
            r#"
local b = buffer.create(128)
local x, y, z, w, u, v = ...

local function writeMany(buf, offset, x, y, z, w, u, v)
    buffer.writef32(buf, offset, x)
    buffer.writef32(buf, offset + 4, y)
    buffer.writef32(buf, offset + 8, z)
    buffer.writef32(buf, offset + 12, w)
    buffer.writef32(buf, offset + 16, u)
    buffer.writef32(buf, offset + 20, v)
end

writeMany(b, 0, x, y, z, w, u, v)
return b
"#
        ),
        r#"
local b = buffer.create(128)
local x, y, z, w, u, v = ...

local function writeMany(buf, offset, x, y, z, w, u, v)
    -- remark: builtin buffer.writef32/3
    buffer.writef32(buf, offset, x)
    -- remark: builtin buffer.writef32/3
    buffer.writef32(buf, offset + 4, y)
    -- remark: builtin buffer.writef32/3
    buffer.writef32(buf, offset + 8, z)
    -- remark: builtin buffer.writef32/3
    buffer.writef32(buf, offset + 12, w)
    -- remark: builtin buffer.writef32/3
    buffer.writef32(buf, offset + 16, u)
    -- remark: builtin buffer.writef32/3
    buffer.writef32(buf, offset + 20, v)
end

-- remark: inlining succeeded (cost 12, profit 1.66x, depth 0)
writeMany(b, 0, x, y, z, w, u, v)
return b
"#
    );
}
