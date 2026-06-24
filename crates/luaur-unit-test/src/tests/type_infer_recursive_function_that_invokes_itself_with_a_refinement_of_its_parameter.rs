//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1427:type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function matches (Analysis/include/Luau/ControlFlow.h)
//!   - translates_to -> rust_item type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter

#[cfg(test)]
#[test]
fn type_infer_recursive_function_that_invokes_itself_with_a_refinement_of_its_parameter() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local TRUE: true = true

        local function matches(value, t: true)
            if value then
                return true
            end
        end

        local function readValue(breakpoint)
            if matches(breakpoint, TRUE) then
                readValue(breakpoint)
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(unknown) -> ()",
            to_string_type_id(fixture.base.require_type_string(&String::from("readValue")))
        );
    } else {
        assert_eq!(
            "<a>(a) -> ()",
            to_string_type_id(fixture.base.require_type_string(&String::from("readValue")))
        );
    }
}
