//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:419:type_infer_tables_open_table_unification_2`
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
//!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_open_table_unification_2

#[cfg(test)]
#[test]
fn type_infer_tables_open_table_unification_2() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::missing_properties::MissingProperties;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = {}
        a.x = 99

        function a:method()
            return self.y
        end
        a:method()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let error = type_error_data_ref::<MissingProperties>(&result.errors[0])
        .expect("expected MissingProperties");
    assert_eq!(1, error.properties().len());
    assert_eq!("y", error.properties()[0].as_str());
    assert_eq!(
        Location {
            begin: Position { line: 7, column: 8 },
            end: Position { line: 7, column: 9 },
        },
        result.errors[0].location
    );
}
