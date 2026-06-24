//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1083:type_function_user_udtf_function_type_cant_call_get_props`
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
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_function_type_cant_call_get_props

#[cfg(test)]
#[test]
fn type_function_user_udtf_function_type_cant_call_get_props() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function hello(arg)
            local arr = arg:properties()
        end
        local function ok(idx: hello<() -> ()>): nil return idx end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
        TypeErrorData::UserDefinedTypeFunctionError(e) => {
            assert_eq!(
                "'hello' type function errored at runtime: [string \"hello\"]:3: type.properties: expected self to be either a table or class, but got function instead",
                e.message()
            );
        }
        other => panic!("expected UserDefinedTypeFunctionError, got {other:?}"),
    }
}
