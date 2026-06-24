//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1146:type_function_user_udtf_calling_each_other_3`
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
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_calling_each_other_3

#[cfg(test)]
#[test]
fn type_function_user_udtf_calling_each_other_3() {
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
        -- this function should not see 'fourth' function when invoked from 'third' that sees it
        type function first(arg)
            return fourth(arg)
        end
        type function second(arg)
            return types.singleton(first(arg))
        end

        do
            type function fourth(arg)
                return arg
            end
            type function third()
                return second("hi")
            end
            local function ok(idx: third<>): nil return idx end
        end
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Unknown global 'fourth'; consider assigning to it first",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "'third' type function errored at runtime: [string \"first\"]:4: attempt to call a nil value",
        to_string_type_error(&result.errors[1])
    );
}
