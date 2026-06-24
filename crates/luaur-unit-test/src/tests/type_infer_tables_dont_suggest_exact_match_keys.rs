//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1533:type_infer_tables_dont_suggest_exact_match_keys`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_infer_tables_dont_suggest_exact_match_keys

#[cfg(test)]
#[test]
fn type_infer_tables_dont_suggest_exact_match_keys() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}
        t.foO = 1
        print(t.Foo)
        t.Foo = 2
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let error = type_error_data_ref::<UnknownPropButFoundLikeProp>(&result.errors[0])
        .expect("expected UnknownPropButFoundLikeProp");
    let t = fixture.base.require_type_string(&String::from("t"));
    assert_eq!(unsafe { follow_type_id(t) }, unsafe {
        follow_type_id(error.table())
    });
    assert_eq!("Foo", error.key());
    assert_eq!(1, error.candidates().len());
    assert!(error.candidates().contains("foO"));
    assert!(!error.candidates().contains("Foo"));
    assert_eq!(
        "Key 'Foo' not found in table 't'.  Did you mean 'foO'?",
        to_string_type_error(&result.errors[0])
    );
}
