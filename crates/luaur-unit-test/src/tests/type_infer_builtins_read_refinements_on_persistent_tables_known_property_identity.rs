//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1838:type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity`
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
//!   - calls -> method MagicInstanceIsA::refine (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - translates_to -> rust_item type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity

#[cfg(test)]
#[test]
fn type_infer_builtins_read_refinements_on_persistent_tables_known_property_identity() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        if bit32.bnot then
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
