//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3085:type_function_user_udtf_areequal_stack_overflow_on_deep_types`
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
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_areequal_stack_overflow_on_deep_types

#[cfg(test)]
#[test]
fn type_function_user_udtf_areequal_stack_overflow_on_deep_types() {
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
        type function deep_eq()
            local depth = 50000
            local function build()
                local t = types.newtable()
                for i = 1, depth do
                    local outer = types.newtable()
                    outer:setproperty(types.singleton("x"), t)
                    t = outer
                end
                return t
            end
            local a = build()
            local b = build()
            if a == b then
                return types.boolean
            end
            return types.string
        end

        local x: deep_eq<> = true
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "'deep_eq' type function errored at runtime: Internal recursion counter limit exceeded in areEqual",
        to_string_type_error(&result.errors[0])
    );
}
