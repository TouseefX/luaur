//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:573:type_infer_oop_cross_module_metatable`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
//!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_infer_oop_cross_module_metatable

#[cfg(test)]
#[test]
fn type_infer_oop_cross_module_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!strict
        local cls = {}
        cls.__index = cls
        function cls:abc() return 4 end
        return cls
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        --!strict
        local cls = require(game.A)
        local tbl = {}
        setmetatable(tbl, cls)
    "#,
        ),
    );

    let module_b_name = String::from("game/B");
    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_b_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_b_name);
    let module_scope = module_b.get_module_scope();
    let cls_binding = module_scope
        .linear_search_for_binding(&String::from("tbl"), false)
        .expect("expected binding for tbl");

    assert_eq!(
        "{ @metatable cls, tbl }",
        to_string_type_id(cls_binding.type_id)
    );
}
