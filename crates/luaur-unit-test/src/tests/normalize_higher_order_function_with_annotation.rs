//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:772:normalize_higher_order_function_with_annotation`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item normalize_higher_order_function_with_annotation

#[cfg(test)]
#[test]
fn normalize_higher_order_function_with_annotation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function apply<a, b>(f: (a) -> b, x)
            return f(x)
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "<a, b>((a) -> b, a) -> b",
        to_string_type_id(fixture.require_type_string(&String::from("apply")))
    );
}
