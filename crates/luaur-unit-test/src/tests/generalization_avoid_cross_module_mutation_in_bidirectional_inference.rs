//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:423:generalization_avoid_cross_module_mutation_in_bidirectional_inference`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item generalization_avoid_cross_module_mutation_in_bidirectional_inference

#[cfg(test)]
#[test]
fn generalization_avoid_cross_module_mutation_in_bidirectional_inference() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::records::module::Module;
    use std::sync::Arc;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("Module/ListFns"),
        String::from(
            r#"
        local mod = {}
        function mod.findWhere(list, predicate): number?
            for i = 1, #list do
                if predicate(list[i], i) then
                    return i
                end
            end
            return nil
        end
        return mod
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        local funs = require(script.Parent.ListFns)
        local accessories = funs.findWhere(getList(), function(accessory)
            return accessory.AccessoryType ~= accessoryTypeEnum
        end)
        return {}
    "#,
        ),
    );

    let module_list_fns = String::from("Module/ListFns");
    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_list_fns, None);
    let mod_list_fns = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_list_fns);

    unsafe {
        let module = Arc::as_ptr(&mod_list_fns) as *mut Module;
        freeze(&mut (*module).interface_types);
        freeze(&mut (*module).internal_types);
    }

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let module_b = String::from("Module/B");
    let _result2 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_b, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
