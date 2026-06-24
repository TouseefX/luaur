//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:497:type_infer_generics_dont_unify_bound_types`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_generics_dont_unify_bound_types

#[cfg(test)]
#[test]
fn type_infer_generics_dont_unify_bound_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type F = <a>() -> <b>(a, b) -> a
        type G = <b>(b, b) -> b
        local f: F = function<a>()
          local x
          return function<b>(y: a, z: b): a
            if not(x) then x = y end
            return x
          end
        end
        -- This assignment shouldn't typecheck
        -- If it does, it means we instantiated
        -- f as () -> <b>(X, b) -> X, then unified X to be b
        local g: G = f()
        -- Oh dear, if that works then the type system is unsound
        local a : string = g("not a number", "hi")
        local b : number = g(5, 37)
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
