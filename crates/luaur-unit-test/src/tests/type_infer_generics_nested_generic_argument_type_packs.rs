//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1103:type_infer_generics_nested_generic_argument_type_packs`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_generics_nested_generic_argument_type_packs

#[cfg(test)]
#[test]
fn type_infer_generics_nested_generic_argument_type_packs() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
function test2(a: number)
    return 3
end

function foo<B...>(f: (B...) -> number, ...: B...)
    return f(...)
end

-- want A... to contain a generic type pack too

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

-- A... = ((B...) -> number, B...))
-- B... = (number)
-- A... = ((number) -> number, number)
wrapper(foo, test2, 3) -- ok
wrapper(foo, test2, 3, 3) -- not ok (too many args)
wrapper(foo, test2) -- not ok (not enough args)
wrapper(foo, test2, "3") -- not ok (type mismatch, string instead of number)
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            Location {
                begin: Position {
                    line: 18,
                    column: 0
                },
                end: Position {
                    line: 18,
                    column: 7
                },
            },
            result.errors[0].location
        );
        let cm = type_error_data_ref::<CountMismatch>(&result.errors[0])
            .expect("expected CountMismatch");
        assert_eq!(3, cm.expected());
        assert_eq!(4, cm.actual());
        assert_eq!(CountMismatch::Arg, cm.context());

        assert_eq!(
            Location {
                begin: Position {
                    line: 19,
                    column: 0
                },
                end: Position {
                    line: 19,
                    column: 7
                },
            },
            result.errors[1].location
        );
        let cm = type_error_data_ref::<CountMismatch>(&result.errors[1])
            .expect("expected CountMismatch");
        assert_eq!(3, cm.expected());
        assert_eq!(2, cm.actual());
        assert_eq!(CountMismatch::Arg, cm.context());

        assert_eq!(
            Location {
                begin: Position {
                    line: 20,
                    column: 20,
                },
                end: Position {
                    line: 20,
                    column: 23,
                },
            },
            result.errors[2].location
        );
        let tm =
            type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
        assert_eq!("number", to_string_type_id(tm.wanted_type));
        assert_eq!("string", to_string_type_id(tm.given_type));
    } else {
        assert_eq!(
            "Argument count mismatch. Function 'wrapper' expects 3 arguments, but 4 are specified",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Argument count mismatch. Function 'wrapper' expects 3 arguments, but only 2 are specified",
            to_string_type_error(&result.errors[1])
        );
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[2])
        );
    }
}
