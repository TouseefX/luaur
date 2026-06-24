//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_native_stack_guard_prevents_stack_overflows() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::FFlag;
    use luaur_common::FInt;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _sff_debug_luau_force_old_solver =
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _sff_luau_use_native_stack_guard =
        ScopedFastFlag::new(&FFlag::LuauUseNativeStackGuard, true);

    let _sff_luau_type_infer_iteration_limit =
        ScopedFastInt::new(&FInt::LuauTypeInferIterationLimit, 0);
    let _sff_luau_stack_guard_threshold =
        ScopedFastInt::new(&FInt::LuauStackGuardThreshold, i32::MAX);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let source = String::from(
        r#"
            local function l0<A...>()
                for l0=_,_ do
                end
            end

            _ = if _._ then function(l0)
            end elseif _._G then if `` then {n0=_,} else "luauExprConstantSt" elseif _[_][l0] then function()
            end elseif _.n0 then if _[_] then if _ then _ else "aeld" elseif false then 0 else "lead"
            return _.n0
        "#,
    );

    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = fixture
            .base
            .check_string_optional_frontend_options(&source, None);
    }));

    let payload = result.expect_err("An expected InternalCompilerError was not thrown");
    let ice = payload
        .downcast_ref::<InternalCompilerError>()
        .expect("expected InternalCompilerError panic payload");
    assert!(
        ice.message.starts_with("Stack overflow in "),
        "{}",
        ice.message
    );
}
