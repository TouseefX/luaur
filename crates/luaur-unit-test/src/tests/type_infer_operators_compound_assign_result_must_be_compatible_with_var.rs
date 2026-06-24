//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:494:type_infer_operators_compound_assign_result_must_be_compatible_with_var`
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
//!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_operators_compound_assign_result_must_be_compatible_with_var

#[cfg(test)]
#[test]
fn type_infer_operators_compound_assign_result_must_be_compatible_with_var() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function __add(left, right)
            return 123
        end

        local mt = {
            __add = __add,
        }

        local x = setmetatable({}, mt)
        local v: number

        v += x -- okay: number + x -> number
        x += v -- not okay: x </: number
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 13,
                column: 8
            },
            end: Position {
                line: 13,
                column: 14
            }
        },
        result.errors[0].location
    );

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("x", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
}
