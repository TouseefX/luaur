//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:418:type_infer_type_instantiations_too_many_provided_type_packs`
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
//!   - type_ref -> record TypeInstantiationCountMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_type_instantiations_too_many_provided_type_packs

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_too_many_provided_type_packs() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
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
        local function f<T...>(): (T...) end

        f<<(string, number), (true, false)>>()
        "#,
            ),
            None,
        );

        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert!(matches!(
            result.errors[0].data,
            TypeErrorData::TypeInstantiationCountMismatch(_)
        ));

        if !FFlag::DebugLuauForceOldSolver.get() {
            assert_eq!(
                "Too many type parameters passed to 'f', which is typed as <T...>(...any) -> (T...). Expected at most 1 type pack, but 2 provided.",
                to_string_type_error(&result.errors[0])
            );
        } else {
            assert_eq!(
                "Too many type parameters passed to 'f', which is typed as <T...>() -> (T...). Expected at most 1 type pack, but 2 provided.",
                to_string_type_error(&result.errors[0])
            );
        }
    }
}
