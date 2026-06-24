//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2300:type_function_user_irreducible_pending_expansions`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - translates_to -> rust_item type_function_user_irreducible_pending_expansions

#[cfg(test)]
#[test]
fn type_function_user_irreducible_pending_expansions() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type function foo(t)
    return types.unionof(t, types.singleton(nil))
end

type table<T> = { a: index<T, "a"> }
type wrap<T> = foo<table<T>>

local x: wrap<{a: number}> = { a = 2 }
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: number }?",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("x")),
            &mut opts
        )
    );
}
