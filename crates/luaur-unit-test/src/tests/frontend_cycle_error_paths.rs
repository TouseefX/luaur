//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:400:frontend_cycle_error_paths`
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
//!   - translates_to -> rust_item frontend_cycle_error_paths

#[cfg(test)]
#[test]
fn frontend_cycle_error_paths() {
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

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let ce1 = type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[0])
        .expect("expected first cycle error");
    assert_eq!("game/Gui/Modules/B", result.errors[0].module_name);
    assert_eq!(2, ce1.cycle().len());
    assert_eq!("game/Gui/Modules/A", ce1.cycle()[0]);
    assert_eq!("game/Gui/Modules/B", ce1.cycle()[1]);

    let ce2 = type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[1])
        .expect("expected second cycle error");
    assert_eq!("game/Gui/Modules/A", result.errors[1].module_name);
    assert_eq!(2, ce2.cycle().len());
    assert_eq!("game/Gui/Modules/B", ce2.cycle()[0]);
    assert_eq!("game/Gui/Modules/A", ce2.cycle()[1]);
}
