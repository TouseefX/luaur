//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:337:type_infer_type_instantiations_metatable_call`
//! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record InstantiateGenericsOnNonFunction (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_type_instantiations_metatable_call

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_metatable_call() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    for enabled in [true, false] {
        let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
        let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
        let mut fixture = BuiltinsFixture::default();
        fixture.get_frontend();

        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
        --!strict
        local t = setmetatable({}, {
            __call = function<T>(self): T
                return nil :: any
            end,
        })

        t<<number>>()
        "#,
            ),
            None,
        );

        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert!(matches!(
            result.errors[0].data,
            TypeErrorData::InstantiateGenericsOnNonFunction(_)
        ));
        assert_eq!(
            "Luau does not currently support explicitly instantiating a table with a `__call` metamethod. You may be able to work around this by creating a function that calls the table, and using that instead.",
            to_string_type_error(&result.errors[0])
        );
    }
}
