//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5240:type_infer_tables_subtyping_with_a_metatable_table_path`
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
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_subtyping_with_a_metatable_table_path

#[cfg(test)]
#[test]
fn type_infer_tables_subtyping_with_a_metatable_table_path() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type self = {} & {}
        type Class = typeof(setmetatable())
        local function _(): Class
            return setmetatable({}::self, {})
        end
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        Location::new(Position::new(2, 21), Position::new(2, 43)),
        result.errors[0].location
    );
    assert_eq!(
        "Type function instance setmetatable<unknown, unknown> is uninhabited",
        to_string_type_error(&result.errors[0])
    );

    assert_eq!(
        Location::new(Position::new(2, 28), Position::new(2, 40)),
        result.errors[1].location
    );
    assert_eq!(
        "Argument count mismatch. Function expects 2 arguments, but none are specified",
        to_string_type_error(&result.errors[1])
    );

    assert_eq!(
        Location::new(Position::new(3, 8), Position::new(5, 11)),
        result.errors[2].location
    );
    assert_eq!(
        "Type function instance setmetatable<unknown, unknown> is uninhabited",
        to_string_type_error(&result.errors[2])
    );

    let expected = "Expected this to be 'setmetatable<unknown, unknown>', but got '{ @metatable {  }, {  } & {  } }'; \n\
the 1st entry in the type pack is `{ @metatable {  }, {  } & {  } }` and in the 1st entry in the type packreduces to \
`never`, and `{ @metatable {  }, {  } & {  } }` is not a subtype of `never`";
    assert_eq!(expected, to_string_type_error(&result.errors[3]));
}
