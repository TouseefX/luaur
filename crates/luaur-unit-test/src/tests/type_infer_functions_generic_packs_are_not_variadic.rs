//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2380:type_infer_functions_generic_packs_are_not_variadic`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_generic_packs_are_not_variadic

#[cfg(test)]
#[test]
fn type_infer_functions_generic_packs_are_not_variadic() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::type_pack_mismatch::TypePackMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function apply<a, b..., c...>(f: (a, b...) -> c..., x: a)
            return f(x)
        end

        local function add(x: number, y: number)
            return x + y
        end

        local function addToSix(x: number)
            return x + 6
        end

        apply(addToSix, 7)
        apply(add, 5)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 21
            },
            end: Position {
                line: 2,
                column: 22
            },
        },
        result.errors[0].location
    );
    let err = unsafe { get_type_error::<TypePackMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypePackMismatch");
    assert_eq!("a", to_string_type_pack_id(err.given_tp()));
    assert_eq!("b...", to_string_type_pack_id(err.wanted_tp()));
}
