//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:790:type_infer_no_stack_overflow_from_isoptional`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record OccursCheckFailed (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_no_stack_overflow_from_isoptional

#[cfg(test)]
#[test]
fn type_infer_no_stack_overflow_from_isoptional() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::occurs_check_failed::OccursCheckFailed;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function _(l0:t0): (any, ()->())
            return 0,_
        end

        type t0 = t0 | {}
        _(nil)
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let t0 = fixture
        .lookup_type(&String::from("t0"))
        .expect("expected type alias t0");
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!("any", to_string_type_id(t0));
    } else {
        assert_eq!("*error-type*", to_string_type_id(t0));
    }

    assert!(
        result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<OccursCheckFailed>(error).is_some()),
        "expected OccursCheckFailed: {:?}",
        result.errors
    );
}
