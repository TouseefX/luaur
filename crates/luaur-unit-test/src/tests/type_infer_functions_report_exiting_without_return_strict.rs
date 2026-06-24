//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:944:type_infer_functions_report_exiting_without_return_strict`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_functions_report_exiting_without_return_strict

#[cfg(test)]
#[test]
fn type_infer_functions_report_exiting_without_return_strict() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local function f1(v): number?
            if v then
                return 1
            end
        end

        local function f2(v)
            if v then
                return 1
            end
        end

        local function f3(v): ()
            if v then
                return
            end
        end

        local function f4(v)
            if v then
                return
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(
        unsafe { get_type_error::<FunctionExitsWithoutReturning>(&result.errors[0]).as_ref() }
            .is_some()
    );
    assert!(
        unsafe { get_type_error::<FunctionExitsWithoutReturning>(&result.errors[1]).as_ref() }
            .is_some()
    );
}
