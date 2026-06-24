//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:415:module_do_not_clone_reexports`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item module_do_not_clone_reexports

#[cfg(test)]
#[test]
fn module_do_not_clone_reexports() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        export type A = {p : number}
        return {}
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        local a = require(script.Parent.A)
        export type B = {q : a.A}
        return {}
    "#,
        ),
    );

    let module_b_name = String::from("Module/B");
    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_b_name, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_a_name = String::from("Module/A");
    let module_a = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_a_name);
    let module_b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&module_b_name);

    let type_a = module_a
        .exported_type_bindings
        .get("A")
        .expect("expected exported type A")
        .r#type();
    let type_b = module_b
        .exported_type_bindings
        .get("B")
        .expect("expected exported type B")
        .r#type();

    let table_b =
        unsafe { get_type_id::<TableType>(type_b).as_ref() }.expect("expected table type B");
    assert_eq!(
        Some(type_a),
        table_b.props.get("q").and_then(|prop| prop.read_ty)
    );
}
