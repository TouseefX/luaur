//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:494:type_infer_refinements_call_an_incompatible_function_after_using_typeguard`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_refinements_call_an_incompatible_function_after_using_typeguard

#[cfg(test)]
#[test]
fn type_infer_refinements_call_an_incompatible_function_after_using_typeguard() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: number)
            return x
        end

        local function g(x: unknown)
            if type(x) == "string" then
                f(x)
            end
        end

        local function h(x: any)
            if type(x) == "string" then
                f(x)
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);

        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            Location::new(Position::new(7, 18), Position::new(7, 19)),
            result.errors[0].location
        );
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);

        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            Location::new(Position::new(7, 18), Position::new(7, 19)),
            result.errors[0].location
        );

        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[1])
        );
        assert_eq!(
            Location::new(Position::new(13, 18), Position::new(13, 19)),
            result.errors[1].location
        );
    }
}
