//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:307:module_clone_self_property`
//! Source: `tests/Module.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Module.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Clone.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Module.test.cpp
//! - outgoing:
//!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item module_clone_self_property

#[cfg(test)]
#[test]
fn module_clone_self_property() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    // CLI-117082 ModuleTests.clone_self_property we don't infer self correctly,
    // instead replacing it with unknown.
    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        --!nonstrict
        local a = {}
        function a:foo(x: number)
            return -x;
        end
        return a;
    "#,
        ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_a, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        --!nonstrict
        local a = require(script.Parent.A)
        return a.foo(5)
    "#,
        ),
    );

    let module_b = String::from("Module/B");
    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_b, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "This function must be called with self. Did you mean to use a colon instead of a dot?",
        to_string_type_error(&result.errors[0])
    );
}
