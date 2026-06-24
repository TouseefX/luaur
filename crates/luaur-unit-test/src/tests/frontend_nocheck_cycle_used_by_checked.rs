//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:318:frontend_nocheck_cycle_used_by_checked`
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
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item frontend_nocheck_cycle_used_by_checked

#[cfg(test)]
#[test]
fn frontend_nocheck_cycle_used_by_checked() {
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
        String::from(
            r#"
        --!nocheck
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
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/C"),
        String::from(
            r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        local B = require(Modules.B)
        return {a=A, b=B}
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/C"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let c_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/Gui/Modules/C"));
    let c_exports = first(c_module.return_type, true).expect("expected C module return type");

    assert_eq!(
        "{ a: { hello: any }, b: { hello: any } }",
        to_string_type_id(c_exports)
    );
}
