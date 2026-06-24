use alloc::string::String;
use alloc::sync::Arc;
use luaur_analysis::enums::polarity::Polarity;
use luaur_analysis::functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
use luaur_analysis::functions::make_function_builtin_definitions::make_function;
use luaur_analysis::functions::persist_type::persist;
use luaur_analysis::functions::unfreeze::unfreeze;
use luaur_analysis::records::binding::Binding;
use luaur_analysis::records::builtin_types::BuiltinTypes;
use luaur_analysis::records::extern_type::ExternType;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::global_types::GlobalTypes;
use luaur_analysis::records::intersection_type::IntersectionType;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::table_indexer::TableIndexer;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::records::type_fun::TypeFun;
use luaur_analysis::records::type_pack::TypePack;
use luaur_analysis::records::union_type::UnionType;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

pub fn register_extern_type_fixture_types(frontend: &mut Frontend) -> (TypeId, TypeId) {
    let builtins = unsafe { &*frontend.builtin_types };
    register_globals(&mut frontend.globals, builtins)
}

fn register_globals(globals: &mut GlobalTypes, builtins: &BuiltinTypes) -> (TypeId, TypeId) {
    unfreeze(globals.global_types_mut());

    let (
        base_class_instance_type,
        base_class_type,
        child_class_instance_type,
        child_class_type,
        grand_child_instance_type,
        another_child_instance_type,
        unrelated_class_instance_type,
        unrelated_class_type,
        vector2_type,
        vector2_instance_type,
        callable_class_type,
        indexable_class_type,
        indexable_numeric_key_class_type,
        duplicate_base_class_instance_type,
        class_with_generic_method_type,
    ) = {
        let arena = globals.global_types_mut();
        let number_type = builtins.numberType;
        let string_type = builtins.stringType;

        let connection_type = arena.add_type(extern_type("Connection", None, None, None));

        let base_class_instance_type = arena.add_type(extern_type("BaseClass", None, None, None));
        let base_method = make_function(
            arena,
            Some(base_class_instance_type),
            alloc::vec![number_type],
            alloc::vec![],
            false,
        );
        let base_class_instance =
            unsafe { get_mutable_type_id::<ExternType>(base_class_instance_type).as_mut() }
                .expect("expected BaseClass instance extern type");
        base_class_instance
            .props
            .insert(String::from("BaseMethod"), Property::readonly(base_method));
        base_class_instance
            .props
            .insert(String::from("BaseField"), Property::rw_type_id(number_type));
        base_class_instance
            .props
            .insert(String::from("Touched"), Property::readonly(connection_type));

        let connect_callback = make_function(
            arena,
            None,
            alloc::vec![base_class_instance_type],
            alloc::vec![],
            false,
        );
        let connect = make_function(
            arena,
            Some(connection_type),
            alloc::vec![connect_callback],
            alloc::vec![],
            false,
        );
        let connection = unsafe { get_mutable_type_id::<ExternType>(connection_type).as_mut() }
            .expect("expected Connection extern type");
        connection
            .props
            .insert(String::from("Connect"), Property::rw_type_id(connect));

        let base_class_type = arena.add_type(extern_type("BaseClass", None, None, None));
        let static_method =
            make_function(arena, None, alloc::vec![], alloc::vec![number_type], false);
        let clone_method = make_function(
            arena,
            None,
            alloc::vec![base_class_instance_type],
            alloc::vec![base_class_instance_type],
            false,
        );
        let base_new = make_function(
            arena,
            None,
            alloc::vec![],
            alloc::vec![base_class_instance_type],
            false,
        );
        let base_class = unsafe { get_mutable_type_id::<ExternType>(base_class_type).as_mut() }
            .expect("expected BaseClass extern type");
        base_class.props.insert(
            String::from("StaticMethod"),
            Property::rw_type_id(static_method),
        );
        base_class
            .props
            .insert(String::from("Clone"), Property::rw_type_id(clone_method));
        base_class
            .props
            .insert(String::from("New"), Property::rw_type_id(base_new));

        let child_class_instance_type = arena.add_type(extern_type(
            "ChildClass",
            Some(base_class_instance_type),
            None,
            None,
        ));
        let child_method = make_function(
            arena,
            Some(child_class_instance_type),
            alloc::vec![],
            alloc::vec![string_type],
            false,
        );
        let child_class_instance =
            unsafe { get_mutable_type_id::<ExternType>(child_class_instance_type).as_mut() }
                .expect("expected ChildClass instance extern type");
        child_class_instance
            .props
            .insert(String::from("Method"), Property::rw_type_id(child_method));

        let child_class_type =
            arena.add_type(extern_type("ChildClass", Some(base_class_type), None, None));
        let child_new = make_function(
            arena,
            None,
            alloc::vec![],
            alloc::vec![child_class_instance_type],
            false,
        );
        let child_class = unsafe { get_mutable_type_id::<ExternType>(child_class_type).as_mut() }
            .expect("expected ChildClass extern type");
        child_class
            .props
            .insert(String::from("New"), Property::rw_type_id(child_new));

        let grand_child_instance_type = arena.add_type(extern_type(
            "GrandChild",
            Some(child_class_instance_type),
            None,
            None,
        ));
        let grand_child_method = make_function(
            arena,
            Some(grand_child_instance_type),
            alloc::vec![],
            alloc::vec![string_type],
            false,
        );
        let grand_child_instance =
            unsafe { get_mutable_type_id::<ExternType>(grand_child_instance_type).as_mut() }
                .expect("expected GrandChild instance extern type");
        grand_child_instance.props.insert(
            String::from("Method"),
            Property::rw_type_id(grand_child_method),
        );

        let another_child_instance_type = arena.add_type(extern_type(
            "AnotherChild",
            Some(base_class_instance_type),
            None,
            None,
        ));
        let another_child_method = make_function(
            arena,
            Some(another_child_instance_type),
            alloc::vec![],
            alloc::vec![string_type],
            false,
        );
        let another_child_instance =
            unsafe { get_mutable_type_id::<ExternType>(another_child_instance_type).as_mut() }
                .expect("expected AnotherChild instance extern type");
        another_child_instance.props.insert(
            String::from("Method"),
            Property::rw_type_id(another_child_method),
        );

        let unrelated_class_instance_type =
            arena.add_type(extern_type("UnrelatedClass", None, None, None));
        let unrelated_class_type = arena.add_type(extern_type("UnrelatedClass", None, None, None));
        let unrelated_new = make_function(
            arena,
            None,
            alloc::vec![],
            alloc::vec![unrelated_class_instance_type],
            false,
        );
        let unrelated_class =
            unsafe { get_mutable_type_id::<ExternType>(unrelated_class_type).as_mut() }
                .expect("expected UnrelatedClass extern type");
        unrelated_class
            .props
            .insert(String::from("New"), Property::rw_type_id(unrelated_new));

        let vector2_meta_type = arena.add_type(TableType::table_type());
        let vector2_instance_type =
            arena.add_type(extern_type("Vector2", None, Some(vector2_meta_type), None));
        let vector2_instance =
            unsafe { get_mutable_type_id::<ExternType>(vector2_instance_type).as_mut() }
                .expect("expected Vector2 instance extern type");
        vector2_instance
            .props
            .insert(String::from("X"), Property::rw_type_id(number_type));
        vector2_instance
            .props
            .insert(String::from("Y"), Property::rw_type_id(number_type));

        let vector2_type = arena.add_type(extern_type("Vector2", None, None, None));
        let vector2_new = make_function(
            arena,
            None,
            alloc::vec![number_type, number_type],
            alloc::vec![vector2_instance_type],
            false,
        );
        let vector2_class = unsafe { get_mutable_type_id::<ExternType>(vector2_type).as_mut() }
            .expect("expected Vector2 extern type");
        vector2_class
            .props
            .insert(String::from("New"), Property::rw_type_id(vector2_new));

        let vector2_add = make_function(
            arena,
            None,
            alloc::vec![vector2_instance_type, vector2_instance_type],
            alloc::vec![vector2_instance_type],
            false,
        );
        let vector2_mul_vector = make_function(
            arena,
            Some(vector2_instance_type),
            alloc::vec![vector2_instance_type],
            alloc::vec![vector2_instance_type],
            false,
        );
        let vector2_mul_number = make_function(
            arena,
            Some(vector2_instance_type),
            alloc::vec![number_type],
            alloc::vec![vector2_instance_type],
            false,
        );
        let vector2_mul = arena.add_type(IntersectionType {
            parts: alloc::vec![vector2_mul_vector, vector2_mul_number],
        });
        let vector2_meta = unsafe { get_mutable_type_id::<TableType>(vector2_meta_type).as_mut() }
            .expect("expected Vector2 metatable");
        vector2_meta
            .props
            .insert(String::from("__add"), Property::rw_type_id(vector2_add));
        vector2_meta
            .props
            .insert(String::from("__mul"), Property::rw_type_id(vector2_mul));

        let callable_class_meta_type = arena.add_type(TableType::table_type());
        let callable_class_type = arena.add_type(extern_type(
            "CallableClass",
            None,
            Some(callable_class_meta_type),
            None,
        ));
        let callable_call = make_function(
            arena,
            None,
            alloc::vec![callable_class_type, string_type],
            alloc::vec![number_type],
            false,
        );
        let callable_meta =
            unsafe { get_mutable_type_id::<TableType>(callable_class_meta_type).as_mut() }
                .expect("expected CallableClass metatable");
        callable_meta
            .props
            .insert(String::from("__call"), Property::rw_type_id(callable_call));

        let indexable_class_meta_type = arena.add_type(TableType::table_type());
        let indexable_key_type = arena.add_type(UnionType {
            options: alloc::vec![string_type, number_type],
        });
        let indexable_class_type = arena.add_type(extern_type(
            "IndexableClass",
            None,
            Some(indexable_class_meta_type),
            Some(TableIndexer {
                index_type: indexable_key_type,
                index_result_type: number_type,
                is_read_only: false,
            }),
        ));

        let indexable_numeric_key_class_meta_type = arena.add_type(TableType::table_type());
        let indexable_numeric_key_class_type = arena.add_type(extern_type(
            "IndexableNumericKeyClass",
            None,
            Some(indexable_numeric_key_class_meta_type),
            Some(TableIndexer {
                index_type: number_type,
                index_result_type: number_type,
                is_read_only: false,
            }),
        ));

        let duplicate_base_class_instance_type = arena.add_type(extern_type(
            "BaseClass",
            Some(base_class_instance_type),
            None,
            None,
        ));
        let duplicate_method = make_function(
            arena,
            Some(duplicate_base_class_instance_type),
            alloc::vec![],
            alloc::vec![string_type],
            false,
        );
        let duplicate_base_class = unsafe {
            get_mutable_type_id::<ExternType>(duplicate_base_class_instance_type).as_mut()
        }
        .expect("expected duplicate BaseClass extern type");
        duplicate_base_class.props.insert(
            String::from("Method"),
            Property::rw_type_id(duplicate_method),
        );

        let generic_t = arena.add_type(GenericType::generic_type_name_polarity(
            &String::from("T"),
            Polarity::Mixed,
        ));
        let identity_args = arena.add_type_pack_t(TypePack::new(alloc::vec![generic_t], None));
        let identity_rets = arena.add_type_pack_t(TypePack::new(alloc::vec![generic_t], None));
        let identity = arena.add_type(FunctionType::new_with_generics(
            alloc::vec![generic_t],
            alloc::vec![],
            identity_args,
            identity_rets,
            None,
            false,
        ));
        let class_with_generic_method_type =
            arena.add_type(extern_type("ClassWithGenericMethod", None, None, None));
        let class_with_generic_method =
            unsafe { get_mutable_type_id::<ExternType>(class_with_generic_method_type).as_mut() }
                .expect("expected ClassWithGenericMethod extern type");
        class_with_generic_method
            .props
            .insert(String::from("identity"), Property::readonly(identity));

        (
            base_class_instance_type,
            base_class_type,
            child_class_instance_type,
            child_class_type,
            grand_child_instance_type,
            another_child_instance_type,
            unrelated_class_instance_type,
            unrelated_class_type,
            vector2_type,
            vector2_instance_type,
            callable_class_type,
            indexable_class_type,
            indexable_numeric_key_class_type,
            duplicate_base_class_instance_type,
            class_with_generic_method_type,
        )
    };

    let module_scope = globals.global_scope();
    let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

    unsafe {
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("BaseClass"),
            TypeFun::type_fun_type_id(base_class_instance_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("ChildClass"),
            TypeFun::type_fun_type_id(child_class_instance_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("GrandChild"),
            TypeFun::type_fun_type_id(grand_child_instance_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("AnotherChild"),
            TypeFun::type_fun_type_id(another_child_instance_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("UnrelatedClass"),
            TypeFun::type_fun_type_id(unrelated_class_instance_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("Vector2"),
            TypeFun::type_fun_type_id(vector2_instance_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("CallableClass"),
            TypeFun::type_fun_type_id(callable_class_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("IndexableClass"),
            TypeFun::type_fun_type_id(indexable_class_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("IndexableNumericKeyClass"),
            TypeFun::type_fun_type_id(indexable_numeric_key_class_type),
        );
        (*module_scope_ptr).exported_type_bindings.insert(
            String::from("ClassWithGenericMethod"),
            TypeFun::type_fun_type_id(class_with_generic_method_type),
        );
    }

    add_global_binding_builtin_definitions_alt_b(globals, "BaseClass", binding(base_class_type));
    add_global_binding_builtin_definitions_alt_b(globals, "ChildClass", binding(child_class_type));
    add_global_binding_builtin_definitions_alt_b(globals, "GrandChild", binding(child_class_type));
    add_global_binding_builtin_definitions_alt_b(
        globals,
        "AnotherChild",
        binding(child_class_type),
    );
    add_global_binding_builtin_definitions_alt_b(
        globals,
        "UnrelatedClass",
        binding(unrelated_class_type),
    );
    add_global_binding_builtin_definitions_alt_b(globals, "Vector2", binding(vector2_type));
    add_global_binding_builtin_definitions_alt_b(
        globals,
        "confusingBaseClassInstance",
        binding(duplicate_base_class_instance_type),
    );
    add_global_binding_builtin_definitions_alt_b(
        globals,
        "ClassWithGenericMethod",
        binding(class_with_generic_method_type),
    );

    for ty in [
        base_class_instance_type,
        child_class_instance_type,
        grand_child_instance_type,
        another_child_instance_type,
        unrelated_class_instance_type,
        vector2_instance_type,
        callable_class_type,
        indexable_class_type,
        indexable_numeric_key_class_type,
        duplicate_base_class_instance_type,
        class_with_generic_method_type,
    ] {
        persist(ty);
    }

    freeze(globals.global_types_mut());
    (vector2_type, vector2_instance_type)
}

fn extern_type(
    name: &str,
    parent: Option<TypeId>,
    metatable: Option<TypeId>,
    indexer: Option<TableIndexer>,
) -> ExternType {
    ExternType {
        name: String::from(name),
        props: Default::default(),
        parent,
        metatable,
        tags: Default::default(),
        user_data: None,
        definition_module_name: String::from("Test"),
        definition_location: None,
        indexer,
        relation: None,
    }
}

fn binding(ty: TypeId) -> Binding {
    Binding {
        type_id: ty,
        location: Location::default(),
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: Some(String::from("@test")),
    }
}
