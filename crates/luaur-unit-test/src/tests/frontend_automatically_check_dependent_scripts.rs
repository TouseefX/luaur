//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:147:frontend_automatically_check_dependent_scripts`
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
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item frontend_automatically_check_dependent_scripts

#[cfg(test)]
#[test]
fn frontend_automatically_check_dependent_scripts() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);

    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/Gui/Modules/B"));
    assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);

    let b_exports = first(b_module.return_type, true).expect("expected module return type");

    assert_eq!("{ b_value: number }", to_string_type_id(b_exports));
}
