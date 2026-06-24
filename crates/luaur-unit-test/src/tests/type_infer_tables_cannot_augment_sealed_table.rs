//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:167:type_infer_tables_cannot_augment_sealed_table`
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
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record CannotExtendTable (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - translates_to -> rust_item type_infer_tables_cannot_augment_sealed_table

#[cfg(test)]
#[test]
fn type_infer_tables_cannot_augment_sealed_table() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::cannot_extend_table::{
        CannotExtendTable, CannotExtendTable_Context,
    };
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function mkt()
            return {prop=999}
        end

        local t = mkt()
        t.foo = 'bar'
    "#,
        ),
        None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        Location {
            begin: Position { line: 6, column: 8 },
            end: Position {
                line: 6,
                column: 13,
            },
        },
        result.errors[0].location
    );

    let error = type_error_data_ref::<CannotExtendTable>(&result.errors[0])
        .expect("expected CannotExtendTable");
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ prop: number }",
        to_string_type_id_to_string_options(error.tableType(), &mut opts)
    );
    assert_eq!("foo", error.prop());
    assert_eq!(CannotExtendTable_Context::Property, error.context());
}
