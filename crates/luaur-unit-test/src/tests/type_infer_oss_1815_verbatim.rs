//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2445:type_infer_oss_1815_verbatim`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_oss_1815_verbatim

#[cfg(test)]
#[test]
fn type_infer_oss_1815_verbatim() {
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
        --!strict
        local item: "foo" = "bar"
        item = if true then "foo" else "foo"

        local item2: "foo" = if true then "doge" else "doge2"
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 28
            },
            end: Position {
                line: 2,
                column: 33
            },
        },
        result.errors[0].location
    );
    let err1 =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("\"foo\"", to_string_type_id(err1.wanted_type));
    assert_eq!("\"bar\"", to_string_type_id(err1.given_type));

    assert_eq!(
        Location {
            begin: Position {
                line: 5,
                column: 42
            },
            end: Position {
                line: 5,
                column: 48
            },
        },
        result.errors[1].location
    );
    let err2 =
        type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!("\"foo\"", to_string_type_id(err2.wanted_type));
    assert_eq!("\"doge\"", to_string_type_id(err2.given_type));

    assert_eq!(
        Location {
            begin: Position {
                line: 5,
                column: 54
            },
            end: Position {
                line: 5,
                column: 61
            },
        },
        result.errors[2].location
    );
    let err3 =
        type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
    assert_eq!("\"foo\"", to_string_type_id(err3.wanted_type));
    assert_eq!("\"doge2\"", to_string_type_id(err3.given_type));
}
