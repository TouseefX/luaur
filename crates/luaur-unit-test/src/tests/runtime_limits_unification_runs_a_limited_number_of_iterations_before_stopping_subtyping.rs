//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_unification_runs_a_limited_number_of_iterations_before_stopping_subtyping() {
    use crate::functions::has_error::has_error;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::normalization_too_complex::NormalizationTooComplex;
    use luaur_common::{FFlag, FInt};

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _limit = ScopedFastInt::new(&FInt::LuauSubtypingIterationLimit, 100);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
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
        ),
        None,
    );

    assert!(
        has_error::<NormalizationTooComplex>(&result),
        "{:?}",
        result.errors
    );
}
