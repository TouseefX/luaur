//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:88:type_infer_type_instantiations_as_stmt_incorrect`
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
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_type_instantiations_as_stmt_incorrect

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_as_stmt_incorrect() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    for enabled in [true, false] {
        let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, !enabled);
        let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
        let mut fixture = Fixture::fixture_bool(false);

        let result = fixture.check_string_optional_frontend_options(
            &String::from(
                r#"
        --!strict
        local function f<T>(a: T, b: T)
            return nil :: any
        end

        f<<number | boolean>>(1, "a")
        "#,
            ),
            None,
        );

        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        if !FFlag::DebugLuauForceOldSolver.get() {
            let expected = "Expected this to be 'boolean | number', but got 'string';\n\
this is because\n\
\t * the 1st component of the union is `number`, and `string` is not a subtype of `number`\n\
\t * the 2nd component of the union is `boolean`, and `string` is not a subtype of `boolean`";
            let actual = to_string_type_error(&result.errors[0]);
            let expected_lines = expected.lines().map(str::trim).collect::<Vec<_>>();
            let actual_lines = actual.lines().map(str::trim).collect::<Vec<_>>();
            assert_eq!(expected_lines, actual_lines);
        } else {
            assert_eq!(
                "Expected this to be 'boolean | number', but got 'string'; none of the union options are compatible",
                to_string_type_error(&result.errors[0])
            );
        }
    }
}
