//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2411:type_function_user_type_alias_values`
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
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - translates_to -> rust_item type_function_user_type_alias_values

#[cfg(test)]
#[test]
fn type_function_user_type_alias_values() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Test = { a: number }

type function foo(t)
    return types.unionof(Test, t)
end

local x: foo<nil> = { a = 2 }
local y: foo<string> = "a"
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: number }?",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("x")),
            &mut x_opts
        )
    );

    let mut y_opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "string | { a: number }",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("y")),
            &mut y_opts
        )
    );
}
