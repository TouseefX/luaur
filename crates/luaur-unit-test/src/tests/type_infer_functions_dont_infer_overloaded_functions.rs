//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2732:type_infer_functions_dont_infer_overloaded_functions`
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
//!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_functions_dont_infer_overloaded_functions

#[cfg(test)]
#[test]
fn type_infer_functions_dont_infer_overloaded_functions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function getR6Attachments(model)
            model:FindFirstChild("Right Leg")
            model:FindFirstChild("Left Leg")
            model:FindFirstChild("Torso")
            model:FindFirstChild("Torso")
            model:FindFirstChild("Head")
            model:FindFirstChild("Left Arm")
            model:FindFirstChild("Right Arm")
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(t1) -> () where t1 = { read FindFirstChild: (t1, string) -> (...unknown) }",
            to_string_type_id(fixture.require_type_string(&String::from("getR6Attachments")))
        );
    } else {
        assert_eq!(
            "<a...>(t1) -> () where t1 = {+ FindFirstChild: (t1, string) -> (a...) +}",
            to_string_type_id(fixture.require_type_string(&String::from("getR6Attachments")))
        );
    }
}
