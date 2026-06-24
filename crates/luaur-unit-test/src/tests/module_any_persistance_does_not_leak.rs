//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:393:module_any_persistance_does_not_leak`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item module_any_persistance_does_not_leak

#[cfg(test)]
#[test]
fn module_any_persistance_does_not_leak() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let module_name = String::from("Module/A");

    fixture.file_resolver.source.insert(
        module_name.clone(),
        String::from(
            r#"
export type A = B
type B = A
    "#,
        ),
    );

    let mut opts = FrontendOptions::default();
    opts.retain_full_type_graphs = false;
    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_name, Some(opts));
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_name);
    let binding = module
        .exported_type_bindings
        .get("A")
        .expect("expected exported type binding A");

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!("any", to_string_type_id(binding.r#type()));
    } else {
        assert_eq!("*error-type*", to_string_type_id(binding.r#type()));
    }
}
