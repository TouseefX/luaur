//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4324:type_infer_tables_read_and_write_only_table_properties_are_unsupported`
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
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method StringWriter::keyword (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_read_and_write_only_table_properties_are_unsupported

#[cfg(test)]
#[test]
fn type_infer_tables_read_and_write_only_table_properties_are_unsupported() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type W = {read x: number}
        type X = {write x: boolean}

        type Y = {read ["prop"]: boolean}
        type Z = {write ["prop"]: string}
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "read keyword is illegal here",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        Location::new(Position::new(1, 18), Position::new(1, 22)),
        result.errors[0].location
    );
    assert_eq!(
        "write keyword is illegal here",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        Location::new(Position::new(2, 18), Position::new(2, 23)),
        result.errors[1].location
    );
    assert_eq!(
        "read keyword is illegal here",
        to_string_type_error(&result.errors[2])
    );
    assert_eq!(
        Location::new(Position::new(4, 18), Position::new(4, 22)),
        result.errors[2].location
    );
    assert_eq!(
        "write keyword is illegal here",
        to_string_type_error(&result.errors[3])
    );
    assert_eq!(
        Location::new(Position::new(5, 18), Position::new(5, 23)),
        result.errors[3].location
    );
}
