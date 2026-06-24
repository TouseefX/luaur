//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2835:type_function_user_type_functions_cannot_try_to_mutate_type_aliases`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_function_user_type_functions_cannot_try_to_mutate_type_aliases

#[cfg(test)]
#[test]
fn type_function_user_type_functions_cannot_try_to_mutate_type_aliases() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _frozen = ScopedFastFlag::new(&FFlag::LuauTypeFunctionSupportsFrozen, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type myType = {}

        type function create_table_with_key()
            myType:setproperty(types.singleton "key", types.optional(types.number))
            return myType
        end
        local my_tbl: create_table_with_key<> = {key = "123"}
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "'create_table_with_key' type function errored at runtime: [string \"create_table_with_key\"]:5: type.setproperty: cannot be called to mutate a frozen type, use `types.copy` to make a copy",
        to_string_type_error(&result.errors[0])
    );
    match &result.errors[1].data {
        TypeErrorData::TypeMismatch(err) => {
            assert_eq!("{ key: string }", to_string_type_id(err.given_type));
            assert_eq!(
                "create_table_with_key<>",
                to_string_type_id(err.wanted_type)
            );
        }
        other => panic!("expected TypeMismatch, got {other:?}"),
    }
}
