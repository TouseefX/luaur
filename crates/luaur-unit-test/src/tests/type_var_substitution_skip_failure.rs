//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:184:type_var_substitution_skip_failure`
//! Source: `tests/TypeVar.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeVar.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeVar.test.cpp
//! - outgoing:
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record Anyification (Analysis/include/Luau/Anyification.h)
//!   - translates_to -> rust_item type_var_substitution_skip_failure

#[cfg(test)]
#[test]
fn type_var_substitution_skip_failure() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::sync::Arc;
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::anyification::Anyification;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::module::Module;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::records::type_pack_var::TypePackVar;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let never_type = builtins.neverType;
    let unknown_type = builtins.unknownType;

    let ftv11 = Type::from(FreeType {
        level: TypeLevel::default(),
        lower_bound: never_type,
        upper_bound: unknown_type,
        ..FreeType::default()
    });
    let ftv11_id = &ftv11 as *const Type;

    let tp24 = TypePackVar::from(TypePack::new(vec![ftv11_id], None));
    let tp17 = TypePackVar::from(TypePack::new(vec![], None));

    let ftv23 = Type::from(FunctionType::function_type_new(&tp24, &tp17, None, false));
    let ftv23_id = &ftv23 as *const Type;

    let mut ttv_connection2 = Type::from(TableType::table_type());
    let ttv_connection2_id = &ttv_connection2 as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_connection2.ty {
        ttv.instantiated_type_params.push(ftv11_id);
        ttv.props
            .insert(String::from("f"), Property::rw_type_id(ftv23_id));
    } else {
        unreachable!();
    }

    let tp21 = TypePackVar::from(TypePack::new(vec![ftv11_id], None));
    let tp20 = TypePackVar::from(TypePack::new(vec![], None));

    let ftv19 = Type::from(FunctionType::function_type_new(&tp21, &tp20, None, false));
    let ftv19_id = &ftv19 as *const Type;

    let mut ttv_signal = Type::from(TableType::table_type());
    let ttv_signal_id = &ttv_signal as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_signal.ty {
        ttv.instantiated_type_params.push(ftv11_id);
        ttv.props
            .insert(String::from("f"), Property::rw_type_id(ftv19_id));
    } else {
        unreachable!();
    }

    if let TypeVariant::Table(ttv) = &mut ttv_connection2.ty {
        ttv.props
            .insert(String::from("signal"), Property::rw_type_id(ttv_signal_id));
    } else {
        unreachable!();
    }

    let gtv_k2 = Type::from(GenericType::default());
    let gtv_k2_id = &gtv_k2 as *const Type;
    let gtv_v2 = Type::from(GenericType::default());
    let gtv_v2_id = &gtv_v2 as *const Type;

    let mut ttv_tween_result2 = Type::from(TableType::table_type());
    let ttv_tween_result2_id = &ttv_tween_result2 as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_tween_result2.ty {
        ttv.instantiated_type_params.push(gtv_k2_id);
        ttv.instantiated_type_params.push(gtv_v2_id);
    } else {
        unreachable!();
    }

    let tp13 = TypePackVar::from(TypePack::new(vec![ttv_tween_result2_id], None));
    let ftv12 = Type::from(FunctionType::function_type_new(&tp13, &tp17, None, false));
    let ftv12_id = &ftv12 as *const Type;

    let mut ttv_connection = Type::from(TableType::table_type());
    let ttv_connection_id = &ttv_connection as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_connection.ty {
        ttv.instantiated_type_params.push(ttv_tween_result2_id);
        ttv.props
            .insert(String::from("f"), Property::rw_type_id(ftv12_id));
        ttv.props
            .insert(String::from("signal"), Property::rw_type_id(ttv_signal_id));
    } else {
        unreachable!();
    }

    let tp9 = TypePackVar::from(TypePack::new(vec![], None));
    let tp10 = TypePackVar::from(TypePack::new(vec![ttv_connection_id], None));

    let ftv8 = Type::from(FunctionType::function_type_new(&tp9, &tp10, None, false));
    let ftv8_id = &ftv8 as *const Type;

    let mut ttv_tween = Type::from(TableType::table_type());
    let ttv_tween_id = &ttv_tween as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_tween.ty {
        ttv.instantiated_type_params.push(gtv_k2_id);
        ttv.instantiated_type_params.push(gtv_v2_id);
        ttv.props
            .insert(String::from("f"), Property::rw_type_id(ftv8_id));
    } else {
        unreachable!();
    }

    let tp4 = TypePackVar::from(TypePack::new(vec![], None));
    let tp5 = TypePackVar::from(TypePack::new(vec![ttv_tween_id], None));

    let ftv3 = Type::from(FunctionType::function_type_new(&tp4, &tp5, None, false));
    let ftv3_id = &ftv3 as *const Type;

    if let TypeVariant::Table(ttv) = &mut ttv_tween_result2.ty {
        ttv.props
            .insert(String::from("f"), Property::rw_type_id(ftv3_id));
    } else {
        unreachable!();
    }

    let gtv_k = Type::from(GenericType::default());
    let gtv_k_id = &gtv_k as *const Type;
    let gtv_v = Type::from(GenericType::default());
    let gtv_v_id = &gtv_v as *const Type;

    let mut ttv_tween_result = Type::from(TableType::table_type());
    let root = &ttv_tween_result as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_tween_result.ty {
        ttv.instantiated_type_params.push(gtv_k_id);
        ttv.instantiated_type_params.push(gtv_v_id);
        ttv.props
            .insert(String::from("f"), Property::rw_type_id(ftv3_id));
    } else {
        unreachable!();
    }

    let mut current_module = Module::default();
    let (global_scope, builtin_types, ice_handler, any_type, any_type_pack) = {
        let frontend = fixture.get_frontend();
        let global_scope = frontend.globals.global_scope();
        let builtin_types = frontend.builtin_types;
        let ice_handler = &mut frontend.ice_handler as *mut _;
        let any_type = unsafe { (*builtin_types).anyType };
        let any_type_pack = unsafe { (*builtin_types).anyTypePack };
        (
            global_scope,
            builtin_types,
            ice_handler,
            any_type,
            any_type_pack,
        )
    };

    let mut anyification =
        Anyification::anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
            &mut current_module.internal_types,
            &Arc::clone(&global_scope),
            builtin_types,
            ice_handler,
            any_type,
            any_type_pack,
        );

    let any = anyification.substitute_type_id(root);

    assert!(!anyification.normalization_too_complex);
    assert!(any.is_some());
    assert_eq!(
        "{ f: t1 } where t1 = () -> { f: () -> { f: ({ f: t1 }) -> (), signal: { f: (any) -> () } } }",
        to_string_type_id(any.unwrap())
    );
}
