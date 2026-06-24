//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1352:type_function_user_udtf_strip_indexer`
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
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> function strip (Common/src/StringUtils.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_function_user_udtf_strip_indexer

#[cfg(test)]
#[test]
fn type_function_user_udtf_strip_indexer() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        type function stripindexer(tbl)
            if not tbl:is("table") then
                error("can only strip the indexer on a table!")
            end
            tbl:setindexer(types.never, types.never)
            return tbl
        end

        type map = { [number]: string, foo: string }
        -- forcing an error here to check the exact type
        local function ok(tbl: stripindexer<map>): never return tbl end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
        TypeErrorData::TypeMismatch(tm) => {
            assert_eq!("{ foo: string }", to_string_type_id(tm.given_type));
        }
        other => panic!("expected TypeMismatch, got {other:?}"),
    }
}
