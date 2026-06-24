//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1254:type_infer_functions_infer_anonymous_function_arguments`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item type_infer_functions_infer_anonymous_function_arguments

#[cfg(test)]
#[test]
fn type_infer_functions_infer_anonymous_function_arguments() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Table = { x: number, y: number }
local function f(a: (Table) -> number) return a({x = 1, y = 2}) end
f(function(a) return a.x + a.y end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Table = { x: number, y: number }
local function f(a: ((Table) -> number)?) if a then return a({x = 1, y = 2}) else return 0 end end
f(function(a) return a.x + a.y end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Table = { x: number, y: number }
local x = {}
x.b = {x = 1, y = 2}
function x:f(a: (Table) -> number) return a(self.b) end
x:f(function(a) return a.x + a.y end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
function f(a: (a: number, b: number, c: boolean) -> number) return a(1, 2, true) end
f(function(a: number, b, c) return c and a + b or b - a end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Table = { x: number, y: number }
local function f(a: (Table) -> number) return a({x = 1, y = 2}) end
f(function(...) return select(1, ...).z end)
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "Key 'z' not found in table 'Table'",
        to_string_type_error(&result.errors[0])
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
function f(a: (a: number, b: number) -> number) return a(1, 2) end
f(function(a, b, c, ...) return a + b end)
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let expected = if FFlag::LuauInstantiateInSubtyping.get() {
        concat!(
            "Expected this to be\n\t",
            "'(number, number) -> number'",
            "\nbut got\n\t",
            "'<a>(number, number, a) -> number'",
            "\ncaused by:\n",
            "  Argument count mismatch. Function expects 3 arguments, but only 2 are specified"
        )
    } else {
        concat!(
            "Expected this to be\n\t",
            "'(number, number) -> number'",
            "\nbut got\n\t",
            "'(number, number, *error-type*) -> number'",
            "\ncaused by:\n",
            "  Argument count mismatch. Function expects 3 arguments, but only 2 are specified"
        )
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
function f(a: (...number) -> number) return a(1, 2) end
f(function(a, b) return a + b end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Table = { x: number, y: number }
function f(a: (...Table) -> number) return a({x = 1, y = 2}, {x = 3, y = 4}) end
f(function(a, ...) local b = ... return b.z end)
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "Key 'z' not found in table 'Table'",
        to_string_type_error(&result.errors[0])
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Table = { x: number, y: number }
function f(a: (number) -> Table) return a(4) end
f(function(x) return x * 2 end)
    "#,
        ),
        None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be 'Table', but got 'number'",
        to_string_type_error(&result.errors[0])
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(a: (number) -> nil) return a(4) end
        f(function(x) print(x) end)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
