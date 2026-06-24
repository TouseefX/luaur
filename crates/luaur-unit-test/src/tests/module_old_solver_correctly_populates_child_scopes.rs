//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:537:module_old_solver_correctly_populates_child_scopes`
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
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
//!   - translates_to -> rust_item module_old_solver_correctly_populates_child_scopes

#[cfg(test)]
#[test]
fn module_old_solver_correctly_populates_child_scopes() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
if true then
end

if false then
end

if true then
else
end

local x = {}
for i,v in x do
end
"#,
        ),
        None,
    );

    let module_name = String::from("MainModule");
    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_name);
    assert_eq!(7, module.get_module_scope().children.len());
}
