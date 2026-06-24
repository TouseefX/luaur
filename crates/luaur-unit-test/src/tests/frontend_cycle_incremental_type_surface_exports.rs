//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:491:frontend_cycle_incremental_type_surface_exports`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_cycle_incremental_type_surface_exports

#[cfg(test)]
#[test]
fn frontend_cycle_incremental_type_surface_exports() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
local b = require(game.B)
export type atype = { x: b.btype }
return {mod_a = 1}
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
export type btype = { x: number }

local function bf()
    local a = require(game.A)
    local bfl : a.atype = nil
    return {bfl.x}
end
return {mod_b = 2}
    "#,
        ),
    );

    let mut result_a = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!result_a.errors.is_empty(), "expected errors");

    let mut result_b = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result_b.errors.is_empty(), "expected errors");

    let module_b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    let ty_b = module_b
        .exported_type_bindings
        .get(&String::from("btype"))
        .expect("expected exported btype")
        .r#type();
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "{ x: number }",
        to_string_type_id_to_string_options(ty_b, &mut opts)
    );

    let module_a = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    let ty_a = module_a
        .exported_type_bindings
        .get(&String::from("atype"))
        .expect("expected exported atype")
        .r#type();
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "{ x: any }",
        to_string_type_id_to_string_options(ty_a, &mut opts)
    );

    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/B"), None);
    result_b = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result_b.errors.is_empty(), "expected errors");

    let module_b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    let ty_b = module_b
        .exported_type_bindings
        .get(&String::from("btype"))
        .expect("expected exported btype")
        .r#type();
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "{ x: number }",
        to_string_type_id_to_string_options(ty_b, &mut opts)
    );

    let module_a = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    let ty_a = module_a
        .exported_type_bindings
        .get(&String::from("atype"))
        .expect("expected exported atype")
        .r#type();
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "{ x: any }",
        to_string_type_id_to_string_options(ty_a, &mut opts)
    );

    let _ = &mut result_a;
}
