//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1391:type_infer_builtins_no_persistent_typelevel_change`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_builtins_no_persistent_typelevel_change

#[cfg(test)]
#[test]
fn type_infer_builtins_no_persistent_typelevel_change() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_mutable_level::get_mutable_level;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    let global_scope = fixture.get_frontend().globals.global_scope();
    let math_ty = fixture.base.require_type_scope_ptr_string(
        global_scope.as_ref() as *const Scope as *mut Scope,
        &String::from("math"),
    );

    let ttv = unsafe { get_mutable_type_id::<TableType>(math_ty).as_mut() }
        .expect("expected math table type");
    let frexp_ty = ttv
        .props
        .get("frexp")
        .and_then(|prop| prop.read_ty)
        .expect("expected math.frexp read type");
    assert!(unsafe { !get_type_id::<FunctionType>(frexp_ty).is_null() });

    let original_level = unsafe {
        let level = get_mutable_level(frexp_ty);
        assert!(!level.is_null());
        *level
    };

    let result = fixture
        .base
        .check_string_optional_frontend_options(&String::from("local a = math.frexp"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let current_level = unsafe {
        let level = get_mutable_level(frexp_ty);
        assert!(!level.is_null());
        *level
    };
    assert_eq!(original_level, current_level);
}
