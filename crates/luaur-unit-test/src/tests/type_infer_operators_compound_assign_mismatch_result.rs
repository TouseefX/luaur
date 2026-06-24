//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:425:type_infer_operators_compound_assign_mismatch_result`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_operators_compound_assign_mismatch_result

#[cfg(test)]
#[test]
fn type_infer_operators_compound_assign_mismatch_result() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local s = 'hello'
        s += 10
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }

    assert_eq!(
        Location {
            begin: Position { line: 2, column: 8 },
            end: Position { line: 2, column: 9 }
        },
        result.errors[0].location
    );
    let number_type = fixture.get_builtins().numberType;
    let string_type = fixture.get_builtins().stringType;
    let tm0 =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(number_type, tm0.wanted_type);
    assert_eq!(string_type, tm0.given_type);

    if FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            Location {
                begin: Position { line: 2, column: 8 },
                end: Position {
                    line: 2,
                    column: 15
                }
            },
            result.errors[1].location
        );
        let tm1 =
            type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
        assert_eq!(string_type, tm1.wanted_type);
        assert_eq!(number_type, tm1.given_type);
    }
}
