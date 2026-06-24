//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1247:type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - translates_to -> rust_item type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds

#[cfg(test)]
#[test]
fn type_infer_operators_equality_operations_succeed_if_any_union_branch_succeeds() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local mm = {}
        type Foo = typeof(setmetatable({}, mm))
        local x: Foo
        local y: Foo?

        local v1 = x == y
        local v2 = y == x
        local v3 = x ~= y
        local v4 = y ~= x
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result2 = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local mm1 = {
            x = "foo",
        }

        local mm2 = {
            y = "bar",
        }

        type Foo = typeof(setmetatable({}, mm1))
        type Bar = typeof(setmetatable({}, mm2))

        local x1: Foo
        local x2: Foo?
        local y1: Bar
        local y2: Bar?

        local v1 = x1 == y1
        local v2 = x2 == y2
    "#,
        ),
        None,
    );

    assert_eq!(1, result2.errors.len(), "{:?}", result2.errors);
    assert_eq!(
        "Types Foo and Bar cannot be compared with == because they do not have the same metatable",
        to_string_type_error(&result2.errors[0])
    );
}
