//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:307:type_infer_type_errors_infer_types`
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
//!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_type_errors_infer_types

#[cfg(test)]
#[test]
fn type_infer_type_errors_infer_types() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::unknown_property::UnknownProperty;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local err = (true).x
        local c = err.Parent.Reward.GetChildren
        local d = err.Parent.Reward
        local e = err.Parent
        local f = err
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = type_error_data_ref::<UnknownProperty>(&result.errors[0])
        .expect("expected UnknownProperty");
    assert_eq!("boolean", to_string_type_id(err.table()));
    assert_eq!("x", err.key());

    if FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "*error-type*",
            to_string_type_id(fixture.require_type_string(&String::from("c")))
        );
        assert_eq!(
            "*error-type*",
            to_string_type_id(fixture.require_type_string(&String::from("d")))
        );
        assert_eq!(
            "*error-type*",
            to_string_type_id(fixture.require_type_string(&String::from("e")))
        );
        assert_eq!(
            "*error-type*",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
    }
}
