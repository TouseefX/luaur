//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3304:type_infer_tables_table_call_metamethod_basic`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_table_call_metamethod_basic

#[cfg(test)]
#[test]
fn type_infer_tables_table_call_metamethod_basic() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = setmetatable({
            a = 1,
        }, {
            __call = function(self, b: number)
                return self.a * b
            end,
        })

        local foo = a(12)
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        type_error_data_ref::<ExplicitFunctionAnnotationRecommended>(&result.errors[0])
            .expect("expected ExplicitFunctionAnnotationRecommended");
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }

    let foo_type = fixture.base.require_type_string(&String::from("foo"));
    let number_type = unsafe { (*fixture.base.builtin_types).numberType };
    assert_eq!(number_type, foo_type);
}
