//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1195:type_function_user_udtf_no_shared_state`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_no_shared_state

#[cfg(test)]
#[test]
fn type_function_user_udtf_no_shared_state() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function foo()
            if not glob then
                glob = 'a'
            else
                glob ..= 'b'
            end

            return glob
        end
        type function bar(prefix)
            return types.singleton(prefix:value() .. foo())
        end
        local function ok1(idx: bar<'x'>): nil return idx end
        local function ok2(idx: bar<'y'>): nil return idx end
    "#,
        ),
        None,
    );

    assert_eq!(5, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Unknown global 'glob'; consider assigning to it first",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "'bar' type function errored at runtime: [string \"foo\"]:4: attempt to modify a readonly table",
        to_string_type_error(&result.errors[1])
    );
}
