//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1501:type_function_user_implicit_export`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_function_user_implicit_export

#[cfg(test)]
#[test]
fn type_function_user_implicit_export() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
type function concat(a: type, b: type)
    local as = a:value()
    local bs = b:value()
    assert(typeof(as) == "string")
    assert(typeof(bs) == "string")
    return types.singleton(as .. bs)
end
export type Concat<T, U> = concat<T, U>
local a: concat<'first', 'second'>
return {}
    "#,
        ),
    );

    let a_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(a_result.errors.is_empty(), "{:?}", a_result.errors);
    assert_eq!(
        "\"firstsecond\"",
        to_string_type_id(
            fixture
                .base
                .require_type_module_name_string("game/A", &String::from("a"))
        )
    );

    let b_result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local Test = require(game.A);
local b: Test.Concat<'third', 'fourth'>
    "#,
        ),
        None,
    );
    assert!(b_result.errors.is_empty(), "{:?}", b_result.errors);
    assert_eq!(
        "\"thirdfourth\"",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
}
