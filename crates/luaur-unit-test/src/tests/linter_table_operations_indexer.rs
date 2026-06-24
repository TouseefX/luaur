//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2179:linter_table_operations_indexer`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item linter_table_operations_indexer

#[cfg(test)]
#[test]
fn linter_table_operations_indexer() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
        &String::from(
            r#"
local t1 = {} -- ok: empty
local t2 = {1, 2} -- ok: array
local t3 = { a = 1, b = 2 } -- not ok: dictionary
local t4: {[number]: number} = {} -- ok: array
local t5: {[string]: number} = {} -- not ok: dictionary
local t6: typeof(setmetatable({1, 2}, {})) = {} -- ok: table with metatable
local t7: string = "hello" -- ok: string
local t8: {number} | {n: number} = {} -- ok: union

-- not ok
print(#t3)
print(#t5)
ipairs(t5)

-- disabled
-- ipairs(t3) adds indexer to t3, silencing error on #t3

-- ok
print(#t1)
print(#t2)
print(#t4)
print(#t6)
print(#t7)
print(#t8)

ipairs(t1)
ipairs(t2)
ipairs(t4)
ipairs(t6)
ipairs(t7)
ipairs(t8)

-- ok, subtle: text is a string here implicitly, but the type annotation isn't available
-- type checker assigns a type of generic table with the 'sub' member; we don't emit warnings on generic tables
-- to avoid generating a false positive here
function _impliedstring(element, text)
        for i = 1, #text do
                element:sendText(text:sub(i, i))
        end
end
"#,
        ),
        None,
    );

    assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(12, result.warnings[0].location.begin.line + 1);
    assert_eq!(
        "Using '#' on a table without an array part is likely a bug",
        result.warnings[0].text.as_str()
    );
    assert_eq!(13, result.warnings[1].location.begin.line + 1);
    assert_eq!(
        "Using '#' on a table with string keys is likely a bug",
        result.warnings[1].text.as_str()
    );
    assert_eq!(14, result.warnings[2].location.begin.line + 1);
    assert_eq!(
        "Using 'ipairs' on a table with string keys is likely a bug",
        result.warnings[2].text.as_str()
    );
}
