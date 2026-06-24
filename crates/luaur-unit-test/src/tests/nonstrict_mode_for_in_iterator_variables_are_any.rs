//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:205:nonstrict_mode_for_in_iterator_variables_are_any`
//! Source: `tests/NonstrictMode.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonstrictMode.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonstrictMode.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item nonstrict_mode_for_in_iterator_variables_are_any

#[cfg(test)]
#[test]
fn nonstrict_mode_for_in_iterator_variables_are_any() {
    use crate::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        function requires_a_table(arg: {}) end
        function requires_a_number(arg: number) end

        local T = {}
        for a, b in pairs(T) do
            requires_a_table(a)
            requires_a_table(b)
            requires_a_number(a)
            requires_a_number(b)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
