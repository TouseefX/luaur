//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5621:type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables`
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
//!   - calls -> method lua_exception::getStatus (VM/src/ldo.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> enum Status (Analysis/src/Linter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables

#[cfg(test)]
#[test]
fn type_infer_tables_generic_index_syntax_bidirectional_infer_with_tables() {
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
        local function getStatus(): string
            return "Yeah can you look in returned books?"
        end
        local function getPratchettStatus()
            return { isLate = true }
        end
        type Status = { isLate: boolean, daysLate: number? }
        local key1 = "Great Expecations"
        local key2 = "The Outsiders"
        local key3 = "Guards! Guards!"
        local books: { [string]: Status } = {
            [key1] = { isLate = true, daysLate = "coconut" },
            [key2] = getStatus(),
            [key3] = getPratchettStatus()
        }
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    let err0 =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err0.given_type));
    assert_eq!("number?", to_string_type_id(err0.wanted_type));
    assert_eq!(
        Location::new(Position::new(12, 49), Position::new(12, 58)),
        result.errors[0].location
    );

    let err1 =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(err1.given_type));
    assert_eq!("Status", to_string_type_id(err1.wanted_type));
    assert_eq!(
        Location::new(Position::new(13, 21), Position::new(13, 32)),
        result.errors[1].location
    );

    let err2 =
        type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
    assert_eq!("{ isLate: boolean }", to_string_type_id(err2.given_type));
    assert_eq!("Status", to_string_type_id(err2.wanted_type));
    assert_eq!(
        Location::new(Position::new(14, 21), Position::new(14, 41)),
        result.errors[2].location
    );
}
