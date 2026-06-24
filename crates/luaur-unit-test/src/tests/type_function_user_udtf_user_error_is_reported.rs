//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1018:type_function_user_udtf_user_error_is_reported`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_function_user_udtf_user_error_is_reported

#[cfg(test)]
#[test]
fn type_function_user_udtf_user_error_is_reported() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function errors_if_string(arg)
            if arg:is("string") then
                local a = 1
                error("We are in a math class! not english")
            end
            return arg
        end
        local function ok(idx: errors_if_string<string>): nil return idx end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
        TypeErrorData::UserDefinedTypeFunctionError(e) => {
            assert_eq!(
                "'errors_if_string' type function errored at runtime: [string \"errors_if_string\"]:5: We are in a math class! not english",
                e.message()
            );
        }
        other => panic!("expected UserDefinedTypeFunctionError, got {other:?}"),
    }
}
