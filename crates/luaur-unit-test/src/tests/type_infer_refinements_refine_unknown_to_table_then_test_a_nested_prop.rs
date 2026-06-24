//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:403:type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop

#[cfg(test)]
#[test]
fn type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::unknown_property::UnknownProperty;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                -- this should error, `x.foo` is an unknown property
                if typeof(x.foo.bar) == "string" then
                    return x.foo.bar
                end
            end

            return nil
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let up = type_error_data_ref::<UnknownProperty>(&result.errors[0])
            .expect("expected UnknownProperty");
        assert_eq!("bar", up.key());
        assert_eq!("unknown", to_string_type_id(up.table()));
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);

        for error in &result.errors {
            let up = type_error_data_ref::<UnknownProperty>(error)
                .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
            assert_eq!("foo", up.key());
            assert_eq!("unknown", to_string_type_id(up.table()));
        }
    }
}
