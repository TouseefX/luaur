//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:179:module_builtin_types_point_into_global_types_arena`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> function isInArena (tests/Fixture.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item module_builtin_types_point_into_global_types_arena

#[cfg(test)]
#[test]
fn module_builtin_types_point_into_global_types_arena() {
    use crate::functions::is_in_arena::is_in_arena;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    let module_name = String::from("MainModule");

    fixture.base.file_resolver.source.insert(
        module_name.clone(),
        String::from(
            r#"
        return {sign=math.sign}
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_name);
    let exports = first(module.return_type, true).expect("expected module return type");

    assert!(is_in_arena(exports, &module.interface_types));

    let exports_table =
        unsafe { get_type_id::<TableType>(exports).as_ref() }.expect("expected return table");

    let sign_type = exports_table
        .props
        .get("sign")
        .and_then(|prop| prop.read_ty)
        .expect("expected sign property read type");

    assert!(!is_in_arena(sign_type, &module.interface_types));
    assert!(is_in_arena(
        sign_type,
        fixture.get_frontend().globals.global_types_mut()
    ));
}
