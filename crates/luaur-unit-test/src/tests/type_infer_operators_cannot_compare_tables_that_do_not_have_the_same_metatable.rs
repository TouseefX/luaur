//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:342:type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable`
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
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable

#[cfg(test)]
#[test]
fn type_infer_operators_cannot_compare_tables_that_do_not_have_the_same_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local M = {}
        function M.new()
            return setmetatable({}, M)
        end
        function M.__lt(left, right) return true end

        local a = M.new()
        local b = {}
        local c = a < b -- line 10
        local d = b < a -- line 11
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        Location {
            begin: Position {
                line: 10,
                column: 18
            },
            end: Position {
                line: 10,
                column: 23
            }
        },
        result.errors[0].location
    );
    assert_eq!(
        Location {
            begin: Position {
                line: 11,
                column: 18
            },
            end: Position {
                line: 11,
                column: 23
            }
        },
        result.errors[1].location
    );
}
