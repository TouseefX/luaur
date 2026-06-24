//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:33:type_infer_primitives_string_index`
//! Source: `tests/TypeInfer.primitives.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record NotATable (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_primitives_string_index

#[cfg(test)]
#[test]
fn type_infer_primitives_string_index() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local s = "Hello, World!"
        local t = s[4]
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let TypeErrorData::NotATable(not_a_table) = &result.errors[0].data else {
        panic!("expected NotATable, got {:?}", result.errors[0]);
    };
    assert_eq!("string", to_string_type_id(not_a_table.ty));

    assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("t")))
    );
}
