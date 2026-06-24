//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1404:type_infer_functions_infer_generic_function_function_argument_overloaded`
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
//!   - translates_to -> rust_item type_infer_functions_infer_generic_function_function_argument_overloaded

#[cfg(test)]
#[test]
fn type_infer_functions_infer_generic_function_function_argument_overloaded() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let mut result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local function g1<T>(a: T, f: (T) -> T) return f(a) end
local function g2<T>(a: T, b: T, f: (T, T) -> T) return f(a, b) end

local g12: typeof(g1) & typeof(g2)

g12(1, function(x) return x + x end)
g12(1, 2, function(x, y) return x + y end)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local function g1<T>(a: T, f: (T) -> T) return f(a) end
local function g2<T>(a: T, b: T, f: (T, T) -> T) return f(a, b) end

local g12: typeof(g1) & typeof(g2)

g12({x=1}, function(x) return {x=-x.x} end)
g12({x=1}, {x=2}, function(x, y) return {x=x.x + y.x} end)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
