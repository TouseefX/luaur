//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:371:frontend_cycle_errors_can_be_fixed`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record ModuleHasCyclicDependency (Analysis/include/Luau/Error.h)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_cycle_errors_can_be_fixed

#[cfg(test)]
#[test]
fn frontend_cycle_errors_can_be_fixed() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from(
            r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
        ),
    );

    let result1 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(2, result1.errors.len(), "{:?}", result1.errors);

    assert!(
        type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[0]).is_some(),
        "Should have been a ModuleHasCyclicDependency: {:?}",
        result1.errors[0]
    );
    assert!(
        type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[1]).is_some(),
        "Should have been a ModuleHasCyclicDependency: {:?}",
        result1.errors[1]
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        return {hello = 42}
    "#,
        ),
    );
    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/Gui/Modules/B"), None);

    let result2 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
}
