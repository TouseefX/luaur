//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3452:type_infer_tables_checked_prop_too_early`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - translates_to -> rust_item type_infer_tables_checked_prop_too_early

#[cfg(test)]
#[test]
fn type_infer_tables_checked_prop_too_early() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t: {x: number?}? = {x = nil}
        local u = t.x and t or 5
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Value of type '{ x: number? }?' could be nil",
        to_string_type_error(&result.errors[0])
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "number | { read x: number, write x: number? }",
            to_string_type_id(fixture.require_type_string(&String::from("u")))
        );
    } else {
        assert_eq!(
            "number | { x: number? }",
            to_string_type_id(fixture.require_type_string(&String::from("u")))
        );
    }
}
