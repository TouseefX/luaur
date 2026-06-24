//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1517:linter_deprecated_api_typed`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
//!   - calls -> function foreach (VM/src/ltablib.cpp)
//!   - calls -> function getn (VM/src/ltablib.cpp)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item linter_deprecated_api_typed

#[cfg(test)]
#[test]
fn linter_deprecated_api_typed() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::collections::BTreeMap;
    use alloc::string::String;
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::get_global_binding::get_global_binding;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::persist_type::persist;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::binding::Binding;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_fun::TypeFun;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_ast::records::location::Location;

    let mut fixture = BuiltinsFixture::default();

    {
        let frontend = fixture.get_frontend();
        unfreeze(frontend.globals.global_types_mut());

        let builtins = unsafe { &mut *frontend.builtin_types };
        let string_type = builtins.stringType;
        let number_type = builtins.numberType;
        let any_type = builtins.anyType;

        let mut instance_props = BTreeMap::new();
        instance_props.insert(String::from("Name"), Property::rw_type_id(string_type));

        let mut data_cost = Property::rw_type_id(number_type);
        data_cost.deprecated = true;
        instance_props.insert(String::from("DataCost"), data_cost);

        let mut wait = Property::rw_type_id(any_type);
        wait.deprecated = true;
        instance_props.insert(String::from("Wait"), wait);

        let instance_type = frontend.globals.global_types_mut().add_type(ExternType {
            name: String::from("Instance"),
            props: instance_props,
            parent: None,
            metatable: None,
            tags: Vec::new(),
            user_data: None,
            definition_module_name: String::from("Test"),
            definition_location: None,
            indexer: None,
            relation: None,
        });

        persist(instance_type);

        let global_scope = frontend.globals.global_scope();
        let scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
        unsafe {
            (*scope_ptr).exported_type_bindings.insert(
                String::from("Instance"),
                TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                    Vec::new(),
                    instance_type,
                    None,
                ),
            );
        }

        let color_type = frontend.globals.global_types_mut().add_type(
            TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                &BTreeMap::new(),
                None,
                TypeLevel::default(),
                scope_ptr,
                TableState::Sealed,
            ),
        );

        let color_table = unsafe { get_mutable_type_id::<TableType>(color_type) };
        if let Some(color_table) = unsafe { color_table.as_mut() } {
            let mut to_hsv = Property::rw_type_id(any_type);
            to_hsv.deprecated = true;
            to_hsv.deprecated_suggestion = String::from("Color3:ToHSV");
            color_table.props.insert(String::from("toHSV"), to_hsv);
        }

        add_global_binding_builtin_definitions_alt_b(
            &mut frontend.globals,
            "Color3",
            Binding {
                type_id: color_type,
                location: Location::default(),
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
            },
        );

        let table_type = get_global_binding(&mut frontend.globals, "table");
        let table = unsafe { get_mutable_type_id::<TableType>(table_type) };
        if let Some(table) = unsafe { table.as_mut() } {
            if let Some(prop) = table.props.get_mut("foreach") {
                prop.deprecated = true;
            }
            if let Some(prop) = table.props.get_mut("getn") {
                prop.deprecated = true;
                prop.deprecated_suggestion = String::from("#");
            }
        }

        freeze(frontend.globals.global_types_mut());
    }

    let result = fixture.base.lint(
        &String::from(
            r#"
return function (i: Instance)
    i:Wait(1.0)
    print(i.Name)
    print(Color3.toHSV())
    print(Color3.doesntexist, i.doesntexist) -- type error, but this verifies we correctly handle non-existent members
    print(table.getn({}))
    table.foreach({}, function() end)
    print(table.nogetn()) -- verify that we correctly handle non-existent members
    return i.DataCost
end
"#,
        ),
        None,
    );

    assert_eq!(5, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Member 'Instance.Wait' is deprecated",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Member 'toHSV' is deprecated, use 'Color3:ToHSV' instead",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "Member 'table.getn' is deprecated, use '#' instead",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "Member 'table.foreach' is deprecated",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        "Member 'Instance.DataCost' is deprecated",
        result.warnings[4].text.as_str()
    );
}
