//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1444:frontend_exported_tables_have_position_metadata`
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
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item frontend_exported_tables_have_position_metadata

#[cfg(test)]
#[test]
fn frontend_exported_tables_have_position_metadata() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        return { abc = 22 }
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let module_scope = module.get_module_scope();
    let return_type = unsafe { (*module_scope).return_type };
    let (ret_head, _) = flatten_type_pack_id(return_type);
    assert_eq!(1, ret_head.len());

    let table = unsafe { get_type_id::<TableType>(ret_head[0]).as_ref() }
        .expect("expected returned table type");

    assert_eq!("MainModule", table.definition_module_name);
    assert_eq!(1, table.props.len());
    let prop = table.props.get("abc").expect("expected abc property");

    assert_eq!(
        Some(Location::new(
            Position {
                line: 1,
                column: 17,
            },
            Position {
                line: 1,
                column: 20,
            },
        )),
        prop.location
    );
}
