//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3055:type_function_user_udtf_deep_copy_iteration_limit_null_deref`
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
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_deep_copy_iteration_limit_null_deref

#[cfg(test)]
#[test]
fn type_function_user_udtf_deep_copy_iteration_limit_null_deref() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::{DFInt, FFlag};

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _robustness = ScopedFastFlag::new(&FFlag::LuauTypeFunctionRobustness, true);
    let _serde_limit = ScopedFastInt::new(&DFInt::LuauTypeFunctionSerdeIterationLimit, 10);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function copy_complex(arg)
            local t = types.newtable()
            t:setproperty(types.singleton("a"), types.number)
            t:setproperty(types.singleton("b"), types.string)
            t:setproperty(types.singleton("c"), types.boolean)
            t:setproperty(types.singleton("d"), types.buffer)
            t:setproperty(types.singleton("e"), types.thread)
            t:setproperty(types.singleton("f"), types.newtable())
            t:setproperty(types.singleton("g"), types.newfunction())
            local c = types.copy(t)
            return c
        end

        local function ok(idx: copy_complex<number>): number return idx end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "'copy_complex' type function errored at runtime: [string \"copy_complex\"]:11: types.copy: complexity limit reached during type copy",
        to_string_type_error(&result.errors[0])
    );
}
