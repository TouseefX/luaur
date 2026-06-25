//! Faithful Rust port of `RefinementExternTypeFixture::getFrontend`'s extern-type
//! registration body (`tests/TypeInfer.refinements.test.cpp` lines 82-146).
//!
//! Adds the Roblox-flavoured extern types the refinement tests need (Vector3,
//! Instance with the magic `IsA`, ExternScriptConnection, Folder, Part,
//! WeldConstraint) to the frontend's global type arena/scope.

use alloc::string::String;
use alloc::sync::Arc;

use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::attach_magic_function::attach_magic_function;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
use luaur_analysis::functions::persist_type::persist;
use luaur_analysis::functions::unfreeze::unfreeze;
use luaur_analysis::records::extern_type::ExternType;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::type_fun::TypeFun;
use luaur_analysis::records::union_type::UnionType;
use luaur_analysis::type_aliases::type_id::TypeId;

use crate::functions::make_magic_instance_is_a::make_magic_instance_is_a;

/// C++ `ExternType{name, {}, parent, std::nullopt, {}, nullptr, "Test", {}}`.
fn extern_type(name: &str, parent: Option<TypeId>) -> ExternType {
    ExternType {
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
    }
}

pub fn register_refinement_extern_type_fixture_types(frontend: &mut Frontend) {
    let builtins = unsafe { &*frontend.builtin_types };
    let number_type = builtins.numberType;
    let string_type = builtins.stringType;
    let boolean_type = builtins.booleanType;
    let nil_type = builtins.nilType;
    let empty_type_pack = builtins.emptyTypePack;
    let root_super = Some(builtins.externType);

    let globals = &mut frontend.globals;
    let scope_ptr = Arc::as_ptr(&globals.global_scope()) as *mut Scope;

    let arena = globals.global_types_mut();
    unfreeze(arena);

    // Vector3 { X: number, Y: number, Z: number }
    let vec3 = arena.add_type(extern_type("Vector3", root_super));
    {
        let v = unsafe { get_mutable_type_id::<ExternType>(vec3).as_mut() }
            .expect("expected Vector3 extern type");
        v.props
            .insert(String::from("X"), Property::rw_type_id(number_type));
        v.props
            .insert(String::from("Y"), Property::rw_type_id(number_type));
        v.props
            .insert(String::from("Z"), Property::rw_type_id(number_type));
    }

    // Instance { Name: string, IsA: (Instance, string) -> boolean (magic) }
    let inst = arena.add_type(extern_type("Instance", root_super));

    let is_a_params = arena.add_type_pack_initializer_list_type_id(&[inst, string_type]);
    let is_a_rets = arena.add_type_pack_initializer_list_type_id(&[boolean_type]);
    let is_a = arena.add_type(FunctionType::function_type_new(
        is_a_params,
        is_a_rets,
        None,
        false,
    ));
    attach_magic_function(is_a, make_magic_instance_is_a());

    {
        let i = unsafe { get_mutable_type_id::<ExternType>(inst).as_mut() }
            .expect("expected Instance extern type");
        i.props
            .insert(String::from("Name"), Property::rw_type_id(string_type));
        i.props
            .insert(String::from("IsA"), Property::rw_type_id(is_a));
    }

    // ExternScriptConnection { Disconnect: (ExternScriptConnection) -> () }
    let script_connection = arena.add_type(extern_type("ExternScriptConnection", Some(inst)));
    let disconnect_args = arena.add_type_pack_initializer_list_type_id(&[script_connection]);
    let disconnect = arena.add_type(FunctionType::function_type_new(
        disconnect_args,
        empty_type_pack,
        None,
        false,
    ));
    {
        let s = unsafe { get_mutable_type_id::<ExternType>(script_connection).as_mut() }
            .expect("expected ExternScriptConnection extern type");
        s.props
            .insert(String::from("Disconnect"), Property::rw_type_id(disconnect));
    }

    // Folder, Part { Position: Vector3 }
    let folder = arena.add_type(extern_type("Folder", Some(inst)));
    let part = arena.add_type(extern_type("Part", Some(inst)));
    {
        let p = unsafe { get_mutable_type_id::<ExternType>(part).as_mut() }
            .expect("expected Part extern type");
        p.props
            .insert(String::from("Position"), Property::rw_type_id(vec3));
    }

    // WeldConstraint { Part0: Part?, Part1: Part? }
    let optional_part = arena.add_type(UnionType {
        options: alloc::vec![part, nil_type],
    });
    let weld_constraint = arena.add_type(extern_type("WeldConstraint", Some(inst)));
    {
        let w = unsafe { get_mutable_type_id::<ExternType>(weld_constraint).as_mut() }
            .expect("expected WeldConstraint extern type");
        w.props
            .insert(String::from("Part0"), Property::rw_type_id(optional_part));
        w.props
            .insert(String::from("Part1"), Property::rw_type_id(optional_part));
    }

    unsafe {
        let exported = &mut (*scope_ptr).exported_type_bindings;
        exported.insert(String::from("Vector3"), TypeFun::type_fun_type_id(vec3));
        exported.insert(String::from("Instance"), TypeFun::type_fun_type_id(inst));
        exported.insert(
            String::from("ExternScriptConnection"),
            TypeFun::type_fun_type_id(script_connection),
        );
        exported.insert(String::from("Folder"), TypeFun::type_fun_type_id(folder));
        exported.insert(String::from("Part"), TypeFun::type_fun_type_id(part));
        exported.insert(
            String::from("WeldConstraint"),
            TypeFun::type_fun_type_id(weld_constraint),
        );

        for (_name, tf) in (*scope_ptr).exported_type_bindings.iter() {
            persist(tf.r#type());
        }
    }

    let mode = if luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::Old
    } else {
        SolverMode::New
    };
    frontend.set_luau_solver_mode(mode);

    freeze(frontend.globals.global_types_mut());
}
