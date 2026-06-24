//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:247:type_infer_type_instantiations_dot_index_call`
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
//!   - translates_to -> rust_item type_infer_type_instantiations_dot_index_call

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_dot_index_call() {
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
        local t = {
            f = function<T>(): T
                return nil :: any
            end,
        }

        local correct: number = t.f<<number>>()
        local incorrect: number = t.f<<string>>()
        "#,
            ),
            None,
        );

        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(9, result.errors[0].location.begin.line);
    }
}
