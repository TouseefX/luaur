//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:572:type_infer_provisional_generic_type_leak_to_module_interface`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record AnyType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_provisional_generic_type_leak_to_module_interface

#[cfg(test)]
#[test]
fn type_infer_provisional_generic_type_leak_to_module_interface() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::any_type::AnyType;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
local wrapStrictTable

local metatable = {
    __index = function(self, key)
        local value = self.__tbl[key]
        if type(value) == "table" then
            -- unification of the free 'wrapStrictTable' with this function type causes generics of this function to leak out of scope
            return wrapStrictTable(value, self.__name .. "." .. key)
        end
        return value
    end,
}

return wrapStrictTable
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
local wrapStrictTable = require(game.A)

local Constants = {}

return wrapStrictTable(Constants, "Constants")
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!("*error-type*", to_string_type_pack_id(module.return_type));
    } else {
        let result = first(module.return_type, true).expect("expected first return type");
        assert!(
            !unsafe { get_type_id::<AnyType>(result) }.is_null(),
            "{:?}",
            result
        );
    }
}
