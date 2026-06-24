//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:795:type_infer_extern_types_read_write_class_properties`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_extern_types_read_write_class_properties

#[cfg(test)]
#[test]
fn type_infer_extern_types_read_write_class_properties() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::binding::Binding;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_analysis::type_aliases::type_id::TypeId;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let (string_type, number_type) = {
        let frontend = fixture.get_frontend();
        let string_type = unsafe { (*frontend.builtin_types).stringType };
        let number_type = unsafe { (*frontend.builtin_types).numberType };

        let globals = &mut frontend.globals;
        unfreeze(globals.global_types_mut());

        let (instance_type, workspace_type, script_type, part_type) = {
            let arena = globals.global_types_mut();
            let make_extern = |name: &str, parent: Option<TypeId>| ExternType {
                name: String::from(name),
                props: Default::default(),
                parent,
                metatable: None,
                tags: Default::default(),
                user_data: None,
                definition_module_name: String::from("Test"),
                definition_location: None,
                indexer: None,
                relation: None,
            };

            let instance_type = arena.add_type(make_extern("Instance", None));
            let instance = unsafe { get_mutable_type_id::<ExternType>(instance_type).as_mut() }
                .expect("expected Instance extern type");
            instance
                .props
                .insert(String::from("Parent"), Property::rw_type_id(instance_type));

            let workspace_type = arena.add_type(make_extern("Workspace", None));

            let script_type = arena.add_type(make_extern("Script", Some(instance_type)));
            let script = unsafe { get_mutable_type_id::<ExternType>(script_type).as_mut() }
                .expect("expected Script extern type");
            script.props.insert(
                String::from("Parent"),
                Property::rw_type_id_type_id(workspace_type, instance_type),
            );

            let part_type = arena.add_type(make_extern("Part", Some(instance_type)));
            let part = unsafe { get_mutable_type_id::<ExternType>(part_type).as_mut() }
                .expect("expected Part extern type");
            part.props.insert(
                String::from("BrickColor"),
                Property::rw_type_id(string_type),
            );
            part.props.insert(
                String::from("Parent"),
                Property::rw_type_id_type_id(workspace_type, instance_type),
            );

            (instance_type, workspace_type, script_type, part_type)
        };

        let workspace = unsafe { get_mutable_type_id::<ExternType>(workspace_type).as_mut() }
            .expect("expected Workspace extern type");
        workspace
            .props
            .insert(String::from("Script"), Property::readonly(script_type));
        workspace
            .props
            .insert(String::from("Part"), Property::readonly(part_type));

        add_global_binding_builtin_definitions_alt_b(
            globals,
            "script",
            Binding {
                type_id: script_type,
                location: Location::default(),
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
            },
        );

        freeze(globals.global_types_mut());
        (string_type, number_type)
    };

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        script.Parent.Part.BrickColor = 0xFFFFFF
        script.Parent.Part.Parent = script
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 1,
                column: 40,
            },
            end: Position {
                line: 1,
                column: 48,
            },
        },
        result.errors[0].location
    );

    let tm = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!(string_type, tm.wanted_type);
    assert_eq!(number_type, tm.given_type);
}
