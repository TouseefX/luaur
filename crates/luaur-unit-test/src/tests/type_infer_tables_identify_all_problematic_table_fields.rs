//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4285:type_infer_tables_identify_all_problematic_table_fields`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_identify_all_problematic_table_fields

#[cfg(test)]
#[test]
fn type_infer_tables_identify_all_problematic_table_fields() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = {
            a: number,
            b: string,
            c: boolean,
        }

        local a: T = {
            a = "foo",
            b = false,
            c = 123,
        }
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    let err0 =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
        Location::new(Position::new(8, 16), Position::new(8, 21)),
        result.errors[0].location
    );
    assert_eq!("string", to_string_type_id(err0.given_type));
    assert_eq!("number", to_string_type_id(err0.wanted_type));

    let err1 =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!(
        Location::new(Position::new(9, 16), Position::new(9, 21)),
        result.errors[1].location
    );
    assert_eq!("boolean", to_string_type_id(err1.given_type));
    assert_eq!("string", to_string_type_id(err1.wanted_type));

    let err2 =
        type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
    assert_eq!(
        Location::new(Position::new(10, 16), Position::new(10, 19)),
        result.errors[2].location
    );
    assert_eq!("number", to_string_type_id(err2.given_type));
    assert_eq!("boolean", to_string_type_id(err2.wanted_type));
}
