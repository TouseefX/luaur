//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:446:module_do_not_clone_types_of_reexported_values`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item module_do_not_clone_types_of_reexported_values

#[cfg(test)]
#[test]
fn module_do_not_clone_types_of_reexported_values() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        local exports = {a={p=5}}
        return exports
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        local a = require(script.Parent.A)
        local exports = {b=a.a}
        return exports
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

    let type_a = first(module_a.return_type, true).expect("expected Module/A return type");
    let type_b = first(module_b.return_type, true).expect("expected Module/B return type");

    let table_a = unsafe { get_type_id::<TableType>(type_a).as_ref() }
        .unwrap_or_else(|| panic!("expected table, got {}", to_string_type_id(type_a)));
    let table_b = unsafe { get_type_id::<TableType>(type_b).as_ref() }
        .unwrap_or_else(|| panic!("expected table, got {}", to_string_type_id(type_b)));

    let prop_a = table_a.props.get("a").expect("expected property a");
    let prop_b = table_b.props.get("b").expect("expected property b");

    assert_eq!(prop_a.read_ty, prop_b.read_ty);
    assert_eq!(prop_a.write_ty, prop_b.write_ty);
}
