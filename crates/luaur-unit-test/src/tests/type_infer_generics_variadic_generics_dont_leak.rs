//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2128:type_infer_generics_variadic_generics_dont_leak`
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
//!   - translates_to -> rust_item type_infer_generics_variadic_generics_dont_leak

#[cfg(test)]
#[test]
fn type_infer_generics_variadic_generics_dont_leak() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function makeApplier<A..., R...>(f: (A...) -> (R...))
            return function (... : A...): R...
                f(...)
            end
        end
        local function add(x: number, y: number): number return x + y end
        local f = makeApplier(add)
    "#,
        ),
        None,
    );

    assert_eq!(
        "(number, number) -> number",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
