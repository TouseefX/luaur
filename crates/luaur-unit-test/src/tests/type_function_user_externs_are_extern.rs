//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2814:type_function_user_externs_are_extern`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_function_user_externs_are_extern

#[cfg(test)]
#[test]
fn type_function_user_externs_are_extern() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
        &String::from(
            r#"
        declare extern type Bar with
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function foo(t: type)
            assert(t.tag == "extern")
            return t
        end

        type T = foo<Bar>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
