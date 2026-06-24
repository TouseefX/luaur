//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:19:type_infer_type_instantiations_as_expression_correct`
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
//!   - translates_to -> rust_item type_infer_type_instantiations_as_expression_correct

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_as_expression_correct() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    for enabled in [true, false] {
        let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
        let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
        let mut fixture = Fixture::fixture_bool(false);

        let result = fixture.check_string_optional_frontend_options(
            &String::from(
                r#"
        --!strict
        local function f<T>(): T
            return nil :: any
        end

        local correct = f<<number>>() + 5
        "#,
            ),
            None,
        );

        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
