//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3156:type_function_user_udtf_setgenerics_wrong_argcount_check`
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
//!   - translates_to -> rust_item type_function_user_udtf_setgenerics_wrong_argcount_check

#[cfg(test)]
#[test]
fn type_function_user_udtf_setgenerics_wrong_argcount_check() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function extra_arg()
            local f = types.newfunction()
            local g = types.generic("T")
            f:setgenerics({g}, "extra")
            return f
        end

        local x: extra_arg<> = nil
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Argument count mismatch. Function expects 1 to 2 arguments, but 3 are specified",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "'extra_arg' type function errored at runtime: [string \"extra_arg\"]:5: type.setgenerics: expected 2 arguments, but got 3",
        to_string_type_error(&result.errors[1])
    );
}
