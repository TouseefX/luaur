//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:453:frontend_cycle_incremental_type_surface_longer`
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
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item frontend_cycle_incremental_type_surface_longer

#[cfg(test)]
#[test]
fn frontend_cycle_incremental_type_surface_longer() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        return {mod_a = 2}
    "#,
        ),
    );

    let mut result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local me = require(game.A)
        return {mod_b = 4}
    "#,
        ),
    );

    result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        local me = require(game.B)
        return {mod_a_prime = 3}
    "#,
        ),
    );

    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/A"), None);
    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/B"), None);

    result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!result.errors.is_empty(), "expected errors");

    let module_a = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    let ty_a = fixture
        .base
        .base
        .require_type_module_ptr_string(&module_a, &String::from("me"));
    assert_eq!("any", to_string_type_id(ty_a));

    result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result.errors.is_empty(), "expected errors");

    let module_b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    let ty_b = fixture
        .base
        .base
        .require_type_module_ptr_string(&module_b, &String::from("me"));
    assert_eq!("any", to_string_type_id(ty_b));
}
