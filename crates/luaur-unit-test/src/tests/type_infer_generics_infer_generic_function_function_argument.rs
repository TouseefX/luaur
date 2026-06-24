//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1389:type_infer_generics_infer_generic_function_function_argument`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_argument

#[cfg(test)]
#[test]
fn type_infer_generics_infer_generic_function_function_argument() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if !FFlag::DebugLuauForceOldSolver.get() {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            local function sum<a>(x: a, y: a, f: (a, a) -> add<a>)
                return f(x, y)
            end
            return sum(2, 3, function<T>(a: T, b: T): add<T> return a + b end)
        "#,
            ),
            None,
        );

        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            local function sum<a>(x: a, y: a, f: (a, a) -> a)
                return f(x, y)
            end
            return sum(2, 3, function(a, b) return a + b end)
        "#,
            ),
            None,
        );

        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
