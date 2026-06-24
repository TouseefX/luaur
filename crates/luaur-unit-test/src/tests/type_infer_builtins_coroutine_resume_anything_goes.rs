//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:532:type_infer_builtins_coroutine_resume_anything_goes`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> function resume (VM/src/ldo.cpp)
//!   - translates_to -> rust_item type_infer_builtins_coroutine_resume_anything_goes

#[cfg(test)]
#[test]
fn type_infer_builtins_coroutine_resume_anything_goes() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function nifty(x, y)
            print(x, y)
            local z = coroutine.yield(1, 2)
            print(z)
            return 42
        end

        local co = coroutine.create(nifty)
        local x, y = coroutine.resume(co, 1, 2)
        local answer = coroutine.resume(co, 3)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
