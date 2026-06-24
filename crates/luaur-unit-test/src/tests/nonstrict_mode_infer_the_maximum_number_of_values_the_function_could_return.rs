//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:45:nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return

#[cfg(test)]
#[test]
fn nonstrict_mode_infer_the_maximum_number_of_values_the_function_could_return() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        function getMinCardCountForWidth(width)
            if width < 513 then
                return 3
            else
                return 8, 'jellybeans'
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let t = fixture.require_type_string(&String::from("getMinCardCountForWidth"));

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!("(number) -> number", to_string_type_id(t));
    } else {
        assert_eq!("(any) -> (...any)", to_string_type_id(t));
    }
}
