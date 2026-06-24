//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:445:type_infer_oop_cycle_between_object_constructor_and_alias`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_oop_cycle_between_object_constructor_and_alias

#[cfg(test)]
#[test]
fn type_infer_oop_cycle_between_object_constructor_and_alias() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::metatable_type::MetatableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local T = {}
        T.__index = T

        function T.new(): T
            return setmetatable({}, T)
        end

        export type T = typeof(T.new())

        return T
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.base.get_main_module(false) };
    let alias_type = module
        .exported_type_bindings
        .get("T")
        .expect("expected exported type T")
        .r#type();
    let followed = unsafe { follow_type_id(alias_type) };
    let metatable = unsafe { get_type_id::<MetatableType>(followed).as_ref() };
    assert!(
        metatable.is_some(),
        "expected metatable type, got {}",
        to_string_type_id(alias_type)
    );
}
