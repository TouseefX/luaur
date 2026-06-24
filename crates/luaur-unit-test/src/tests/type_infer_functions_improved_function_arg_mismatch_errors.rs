//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1975:type_infer_functions_improved_function_arg_mismatch_errors`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_functions_improved_function_arg_mismatch_errors

#[cfg(test)]
#[test]
fn type_infer_functions_improved_function_arg_mismatch_errors() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local function foo1(a: number) end
foo1()

local function foo2(a: number, b: string?) end
foo2()

local function foo3(a: number, b: string?, c: any) end -- any is optional
foo3()

string.find()

local t = {}
function t.foo(x: number, y: string?, ...: any) return 1 end
function t:bar(x: number, y: string?) end
t.foo()

t:bar()

local u = { a = t, b = function() return t end }
u.a.foo()
local x = (u.a).foo()

u.b().foo()
    "#,
        ),
        None,
    );

    assert_eq!(9, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        [
            "Argument count mismatch. Function expects 1 argument, but none are specified",
            "Argument count mismatch. Function expects 1 to 2 arguments, but none are specified",
            "Argument count mismatch. Function expects 1 to 3 arguments, but none are specified",
            "Argument count mismatch. Function expects 2 to 4 arguments, but none are specified",
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
            "Argument count mismatch. Function expects 2 to 3 arguments, but only 1 is specified",
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
        ]
    } else {
        [
            "Argument count mismatch. Function 'foo1' expects 1 argument, but none are specified",
            "Argument count mismatch. Function 'foo2' expects 1 to 2 arguments, but none are specified",
            "Argument count mismatch. Function 'foo3' expects 1 to 3 arguments, but none are specified",
            "Argument count mismatch. Function 'string.find' expects 2 to 4 arguments, but none are specified",
            "Argument count mismatch. Function 't.foo' expects at least 1 argument, but none are specified",
            "Argument count mismatch. Function 't.bar' expects 2 to 3 arguments, but only 1 is specified",
            "Argument count mismatch. Function 'u.a.foo' expects at least 1 argument, but none are specified",
            "Argument count mismatch. Function 'u.a.foo' expects at least 1 argument, but none are specified",
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
        ]
    };

    for (error, expected) in result.errors.iter().zip(expected) {
        assert_eq!(expected, to_string_type_error(error));
    }
}
