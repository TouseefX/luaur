//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1847:type_infer_builtins_read_refinements_on_persistent_tables_unknown_property`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_builtins_read_refinements_on_persistent_tables_unknown_property

#[cfg(test)]
#[test]
fn type_infer_builtins_read_refinements_on_persistent_tables_unknown_property() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::unknown_property::UnknownProperty;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        if bit32.scrambleEggs then
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<UnknownProperty>(&result.errors[0]).as_ref() }
        .expect("expected UnknownProperty");
    assert_eq!("scrambleEggs", err.key());
    assert_eq!("typeof(bit32)", to_string_type_id(err.table()));
}
