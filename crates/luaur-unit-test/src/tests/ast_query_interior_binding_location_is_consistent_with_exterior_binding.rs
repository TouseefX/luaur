//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:402:ast_query_interior_binding_location_is_consistent_with_exterior_binding`
//! Source: `tests/AstQuery.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstQuery.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file tests/AstQueryDsl.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/AstQuery.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function findBindingAtPosition (Analysis/src/AstQuery.cpp)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item ast_query_interior_binding_location_is_consistent_with_exterior_binding

#[cfg(test)]
#[test]
fn ast_query_interior_binding_location_is_consistent_with_exterior_binding() {
    use crate::tests::ast_query_support::*;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function abcd(arg)
            abcd(arg)
        end

        abcd(0)
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty());

    let module = fixture.get_main_module(false);
    let source_module = fixture.get_main_source_module();

    let decl_binding = unsafe {
        find_binding_at_position(
            &*module,
            &*source_module,
            Position {
                line: 1,
                column: 26,
            },
        )
    };
    assert!(decl_binding.is_some());
    assert_eq!(
        decl_binding.unwrap().location,
        Location {
            begin: Position {
                line: 1,
                column: 23
            },
            end: Position {
                line: 1,
                column: 27
            },
        }
    );

    let inner_call_binding = unsafe {
        find_binding_at_position(
            &*module,
            &*source_module,
            Position {
                line: 2,
                column: 15,
            },
        )
    };
    assert!(inner_call_binding.is_some());
    assert_eq!(
        inner_call_binding.unwrap().location,
        Location {
            begin: Position {
                line: 1,
                column: 23
            },
            end: Position {
                line: 1,
                column: 27
            },
        }
    );

    let outer_call_binding = unsafe {
        find_binding_at_position(&*module, &*source_module, Position { line: 5, column: 8 })
    };
    assert!(outer_call_binding.is_some());
    assert_eq!(
        outer_call_binding.unwrap().location,
        Location {
            begin: Position {
                line: 1,
                column: 23
            },
            end: Position {
                line: 1,
                column: 27
            },
        }
    );
}
