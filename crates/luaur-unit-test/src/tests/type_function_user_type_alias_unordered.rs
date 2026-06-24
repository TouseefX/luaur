//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2432:type_function_user_type_alias_unordered`
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
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - translates_to -> rust_item type_function_user_type_alias_unordered

#[cfg(test)]
#[test]
fn type_function_user_type_alias_unordered() {
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
type function foobar(ty)
    if ty:is("number") then
        return ty
    end
    return TableOf(ty)
end

type TableOf<T> = { prop: T }

type ShouldBeNumber = foobar<number>
type ShouldBeTableOfString = foobar<string>

local x: ShouldBeNumber = 2
local y: ShouldBeTableOfString = { prop = "a" }
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let mut x_opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "number",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("x")),
            &mut x_opts
        )
    );

    let mut y_opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ prop: string }",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("y")),
            &mut y_opts
        )
    );
}
