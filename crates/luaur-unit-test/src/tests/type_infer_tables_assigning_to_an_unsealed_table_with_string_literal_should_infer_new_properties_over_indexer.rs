//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1049:type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer

#[cfg(test)]
#[test]
fn type_infer_tables_assigning_to_an_unsealed_table_with_string_literal_should_infer_new_properties_over_indexer(
) {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}
        t["a"] = "foo"

        local a = t.a
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.get_builtins().stringType)
    );

    let t_type = fixture.require_type_string(&String::from("t"));
    let table_type = unsafe { get_type_id::<TableType>(t_type).as_ref() }.unwrap_or_else(|| {
        panic!("Expected a table but got {}", to_string_type_id(t_type));
    });

    assert!(table_type.indexer.is_none(), "{:?}", table_type.indexer);
    assert!(table_type.props.contains_key("a"));

    let a = table_type.props.get("a").expect("expected property a");
    let property_a = a.read_ty.expect("expected read type for property a");
    assert_eq!("string", to_string_type_id(property_a));
}
