//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:148:type_infer_type_instantiations_anonymous_type_inferred`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_type_instantiations_anonymous_type_inferred

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_anonymous_type_inferred() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    for enabled in [true, false] {
        let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
        let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
        let mut fixture = Fixture::fixture_bool(false);

        let result = fixture.check_string_optional_frontend_options(
            &String::from(
                r#"
        --!strict
        local function f<T, U>(): { a: T, b: U }
            return nil :: any
        end

        local correct: { a: number, b: string } = f<<number>>()
        local incorrect: { a: number, b: string } = f<<string>>()
        "#,
            ),
            None,
        );

        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(7, result.errors[0].location.begin.line);
        assert!(matches!(
            result.errors[0].data,
            TypeErrorData::TypeMismatch(_)
        ));
    }
}
