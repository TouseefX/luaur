//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5594:type_infer_tables_missing_fields_bidirectional_inference`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_missing_fields_bidirectional_inference

#[cfg(test)]
#[test]
fn type_infer_tables_missing_fields_bidirectional_inference() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::missing_properties::MissingProperties;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Book = { title: string, author: string }
        local b: Book = { title = "The Odyssey" }
        local t: { Book } = {
            { title = "The Illiad", author = "Homer" },
            { title = "Inferno", author = "Virgil" },
            { author = "Virgil" },
        }
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let err0 = type_error_data_ref::<MissingProperties>(&result.errors[0])
        .expect("expected MissingProperties");
    assert_eq!(1, err0.properties().len());
    assert_eq!("author", err0.properties()[0].as_str());
    assert_eq!(
        Location::new(Position::new(2, 24), Position::new(2, 49)),
        result.errors[0].location
    );

    let err1 = type_error_data_ref::<MissingProperties>(&result.errors[1])
        .expect("expected MissingProperties");
    assert_eq!(1, err1.properties().len());
    assert_eq!("title", err1.properties()[0].as_str());
    assert_eq!(
        Location::new(Position::new(6, 12), Position::new(6, 33)),
        result.errors[1].location
    );
}
