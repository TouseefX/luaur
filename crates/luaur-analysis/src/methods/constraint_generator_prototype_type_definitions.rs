//! Faithful 1:1 port of ConstraintGenerator::prototypeTypeDefinitions
//! (luau/Analysis/src/ConstraintGenerator.cpp lines 820-1274).
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ffi::CStr;
use core::ptr::NonNull;

use luaur_common::FFlag;

use crate::records::blocked_type::BlockedType;
use crate::records::built_in_type_function_error::BuiltInTypeFunctionError;
use crate::records::class_decl_record::ClassDeclRecord;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::duplicate_type_definition::DuplicateTypeDefinition;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::generic_type::GenericType;
use crate::records::klass::Klass;
use crate::records::obj::Obj;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::records::table_type::TableType;
use crate::records::type_fun::TypeFun;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_level::TypeLevel;
use crate::records::user_defined_function_data::UserDefinedFunctionData;

use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;

use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_valid_class_metamethod::is_valid_class_metamethod;

use crate::type_aliases::name_type::Name;
use crate::type_aliases::nominal_relation::NominalRelation;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;

use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_ast::records::location::Location;

use crate::records::global_name_collector::GlobalNameCollector;
use crate::type_aliases::type_id::TypeId;

impl ConstraintGenerator {
    pub fn prototype_type_definitions(&mut self, scope: *mut Scope, block: *mut AstStatBlock) {
        // Reconstruct a borrowable ScopePtr from the raw pointer without
        // consuming the refcount.
        let sp = core::mem::ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });

        // DenseHashMap<Name, Location> typeNameLocations{Name{}};
        let mut type_name_locations: BTreeMap<String, Location> = BTreeMap::new();

        // TODO: Clip these when clipping FFlag::LuauTidyTypePrototyping
        let mut deprecated_alias_definition_locations: BTreeMap<String, Location> = BTreeMap::new();
        let mut deprecated_class_definition_locations: BTreeMap<String, Location> = BTreeMap::new();

        let mut has_type_function = false;
        let mut type_function_env_scope: Option<ScopePtr> = None;

        // globalScope (used in the LuauDisallowRedefiningBuiltinTypes and env passes).
        let global_scope = self.global_scope.clone().unwrap();

        // In order to enable mutually-recursive type aliases, we need to
        // populate the type bindings before we actually check any of the
        // alias statements.
        let body = unsafe { (*block).body };
        for i in 0..body.size {
            let stat = unsafe { *body.data.add(i) };
            let node = stat as *mut AstNode;

            // if (auto alias = stat->as<AstStatTypeAlias>())
            let alias = unsafe { (*node).as_item_mut::<AstStatTypeAlias>() };
            if !alias.is_null() {
                let alias_name = unsafe { (*alias).name };
                let alias_name_str = ast_name_to_string(alias_name);
                let alias_location = unsafe { (*alias).base.base.location };

                if FFlag::LuauDisallowRedefiningBuiltinTypes.get()
                    && unsafe {
                        (*(global_scope.as_ref() as *const Scope as *mut Scope))
                            .builtin_type_names
                            .contains(&alias_name_str)
                    }
                {
                    self.report_error(
                        alias_location,
                        TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                            alias_name_str.clone(),
                            None,
                        )),
                    );
                    continue;
                }

                if FFlag::LuauTidyTypePrototyping.get() {
                    // A type alias might have no name if the code is syntactically
                    // illegal. We mustn't prepopulate anything in this case.
                    if ast_name_is(alias_name, b"%error-id%") || ast_name_is(alias_name, b"typeof")
                    {
                        continue;
                    }

                    if let Some(loc) = type_name_locations.get(&alias_name_str) {
                        let loc = *loc;
                        self.report_error(
                            alias_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                alias_name_str.clone(),
                                Some(loc),
                            )),
                        );
                        continue;
                    }
                } else {
                    if unsafe {
                        (*scope)
                            .exported_type_bindings
                            .contains_key(&alias_name_str)
                            || (*scope).private_type_bindings.contains_key(&alias_name_str)
                    } {
                        let it = deprecated_alias_definition_locations.get(&alias_name_str);
                        debug_assert!(it.is_some());
                        let prev = it.copied();
                        self.report_error(
                            alias_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                alias_name_str.clone(),
                                prev,
                            )),
                        );
                        continue;
                    }

                    // A type alias might have no name if the code is syntactically
                    // illegal. We mustn't prepopulate anything in this case.
                    if ast_name_is(alias_name, b"%error-id%") || ast_name_is(alias_name, b"typeof")
                    {
                        continue;
                    }
                }

                let defn_scope = self.child_scope(alias as *mut AstNode, &sp);

                let initial_type = unsafe { (*self.arena).add_type(BlockedType::default()) };
                let mut initial_fun = TypeFun::type_fun_type_id(initial_type);

                /* The boolean toggle `addTypes` decides whether or not to introduce the generic
                type/pack param into the privateType/Pack bindings. See C++ comment. */
                let generics = unsafe { (*alias).generics };
                for (_name, gen) in self.create_generics(&defn_scope, generics, true, false) {
                    initial_fun.type_params.push(gen);
                }

                let generic_packs = unsafe { (*alias).generic_packs };
                for (_name, gen_pack) in
                    self.create_generic_packs(&defn_scope, generic_packs, true, false)
                {
                    initial_fun.type_pack_params.push(gen_pack);
                }
                initial_fun.definition_location = Some(alias_location);

                if unsafe { (*alias).exported } {
                    unsafe {
                        (*scope)
                            .exported_type_bindings
                            .insert(alias_name_str.clone(), initial_fun);
                    }
                } else {
                    unsafe {
                        (*scope)
                            .private_type_bindings
                            .insert(alias_name_str.clone(), initial_fun);
                    }
                }

                *self
                    .ast_type_alias_defining_scopes
                    .get_or_insert(alias as *const AstStatTypeAlias) = Some(defn_scope.clone());
                if FFlag::LuauTidyTypePrototyping.get() {
                    type_name_locations.insert(alias_name_str.clone(), alias_location);
                } else {
                    deprecated_alias_definition_locations
                        .insert(alias_name_str.clone(), alias_location);
                }
                continue;
            }

            // else if (auto function = stat->as<AstStatTypeFunction>())
            let function = unsafe { (*node).as_item_mut::<AstStatTypeFunction>() };
            if !function.is_null() {
                has_type_function = true;

                let function_name = unsafe { (*function).name };
                let function_name_str = ast_name_to_string(function_name);
                let function_location = unsafe { (*function).base.base.location };

                // If a type function w/ same name has already been defined, error for having duplicates
                if FFlag::LuauTidyTypePrototyping.get() {
                    if let Some(loc) = type_name_locations.get(&function_name_str) {
                        let loc = *loc;
                        self.report_error(
                            function_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                function_name_str.clone(),
                                Some(loc),
                            )),
                        );
                        continue;
                    }
                } else {
                    if unsafe {
                        (*scope)
                            .exported_type_bindings
                            .contains_key(&function_name_str)
                            || (*scope)
                                .private_type_bindings
                                .contains_key(&function_name_str)
                    } {
                        let it = deprecated_alias_definition_locations.get(&function_name_str);
                        debug_assert!(it.is_some());
                        let prev = it.copied();
                        self.report_error(
                            function_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                function_name_str.clone(),
                                prev,
                            )),
                        );
                        continue;
                    }
                }

                // Create TypeFunctionInstanceType
                let body_fn = unsafe { (*function).body };
                let args = unsafe { (*body_fn).args };
                let args_size = args.size;

                let mut type_params: Vec<TypeId> = Vec::new();
                type_params.reserve(args_size);

                let mut quantified_type_params: Vec<
                    crate::records::generic_type_definition::GenericTypeDefinition,
                > = Vec::new();
                quantified_type_params.reserve(args_size);

                for j in 0..args_size {
                    let name: String = format!("T{}", j);
                    let ty = unsafe {
                        (*self.arena).add_type(GenericType::generic_type_name_polarity(
                            &name,
                            Polarity::Unknown,
                        ))
                    };
                    type_params.push(ty);

                    let generic_ty =
                        crate::records::generic_type_definition::GenericTypeDefinition {
                            ty,
                            defaultValue: None,
                        };
                    quantified_type_params.push(generic_ty);
                }

                if FFlag::LuauTypeFunctionStructuredErrors.get() {
                    if let Some(error) =
                        unsafe { (*self.type_function_runtime).register_function(function) }
                    {
                        self.report_error(
                            function_location,
                            TypeErrorData::BuiltInTypeFunctionError(BuiltInTypeFunctionError {
                                error,
                            }),
                        );
                    }
                } else {
                    if let Some(error) = unsafe {
                        (*self.type_function_runtime).register_function_deprecated(function)
                    } {
                        self.report_error(
                            function_location,
                            TypeErrorData::GenericError(GenericError::new(error)),
                        );
                    }
                }

                let mut udtf_data =
                    UserDefinedFunctionData::new(Arc::downgrade(self.module.as_ref().unwrap()));
                // udtfData.owner = module; (set above) ; udtfData.definition = function;
                udtf_data.definition = function;

                let user_func =
                    NonNull::from(unsafe { &(*self.builtin_types).typeFunctions.user_func });
                let type_function_ty = unsafe {
                    (*self.arena).add_type(TypeFunctionInstanceType::new(
                        user_func,
                        type_params,
                        Vec::new(),
                        Some(function_name),
                        udtf_data,
                    ))
                };

                let mut type_function =
                    TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                        quantified_type_params,
                        type_function_ty,
                        None,
                    );

                type_function.definition_location = Some(function_location);

                // Set type bindings and definition locations for this user-defined type function
                if unsafe { (*function).exported } {
                    unsafe {
                        (*scope)
                            .exported_type_bindings
                            .insert(function_name_str.clone(), type_function);
                    }
                } else {
                    unsafe {
                        (*scope)
                            .private_type_bindings
                            .insert(function_name_str.clone(), type_function);
                    }
                }

                if FFlag::LuauTidyTypePrototyping.get() {
                    type_name_locations.insert(function_name_str.clone(), function_location);
                } else {
                    deprecated_alias_definition_locations
                        .insert(function_name_str.clone(), function_location);
                }
                continue;
            }

            // else if (auto classDeclaration = stat->as<AstStatDeclareExternType>())
            let class_declaration = unsafe { (*node).as_item_mut::<AstStatDeclareExternType>() };
            if !class_declaration.is_null() {
                let class_decl_name = unsafe { (*class_declaration).name };
                let class_decl_name_str = ast_name_to_string(class_decl_name);
                let class_decl_location = unsafe { (*class_declaration).base.base.location };

                if FFlag::LuauTidyTypePrototyping.get() {
                    // A class might have no name if the code is syntactically illegal.
                    if ast_name_is(class_decl_name, b"%error-id%") {
                        continue;
                    }

                    if let Some(loc) = type_name_locations.get(&class_decl_name_str) {
                        let loc = *loc;
                        self.report_error(
                            class_decl_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                class_decl_name_str.clone(),
                                Some(loc),
                            )),
                        );
                        continue;
                    }
                } else {
                    if unsafe {
                        (*scope)
                            .exported_type_bindings
                            .contains_key(&class_decl_name_str)
                    } {
                        let it = deprecated_class_definition_locations.get(&class_decl_name_str);
                        debug_assert!(it.is_some());
                        let prev = it.copied();
                        self.report_error(
                            class_decl_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                class_decl_name_str.clone(),
                                prev,
                            )),
                        );
                        continue;
                    }

                    // A class might have no name if the code is syntactically illegal.
                    if ast_name_is(class_decl_name, b"%error-id%") {
                        continue;
                    }
                }

                let defn_scope = self.child_scope(class_declaration as *mut AstNode, &sp);
                let _ = defn_scope; // mirrors C++ (defnScope is created but unused beyond this)

                let initial_type = unsafe { (*self.arena).add_type(BlockedType::default()) };
                let mut initial_fun = TypeFun::type_fun_type_id(initial_type);
                initial_fun.definition_location = Some(class_decl_location);
                unsafe {
                    (*scope)
                        .exported_type_bindings
                        .insert(class_decl_name_str.clone(), initial_fun);
                }

                if FFlag::LuauTidyTypePrototyping.get() {
                    type_name_locations.insert(class_decl_name_str.clone(), class_decl_location);
                } else {
                    deprecated_class_definition_locations
                        .insert(class_decl_name_str.clone(), class_decl_location);
                }
                continue;
            }

            // else if (auto classDecl = stat->as<AstStatClass>())
            let class_decl = unsafe { (*node).as_item_mut::<AstStatClass>() };
            if !class_decl.is_null() {
                debug_assert!(FFlag::DebugLuauUserDefinedClasses.get());

                let name_local = unsafe { (*class_decl).name };
                let decl_name: Name = ast_name_to_string(unsafe { (*name_local).name });
                let the_def = unsafe { (*self.dfg).get_def_for_local(name_local) };
                let class_decl_location = unsafe { (*class_decl).base.base.location };
                let name_local_location = unsafe { (*name_local).location };

                let error_type = unsafe { (*self.builtin_types).errorType };

                if FFlag::LuauTidyTypePrototyping.get() {
                    if let Some(loc) = type_name_locations.get(&decl_name) {
                        let loc = *loc;
                        self.report_error(
                            class_decl_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                decl_name.clone(),
                                Some(loc),
                            )),
                        );
                        unsafe {
                            (*scope).bindings.insert(
                                Symbol::from_local(name_local),
                                make_binding(error_type, class_decl_location),
                            );
                            *(*scope).lvalue_types.get_or_insert(the_def) = error_type;
                        }
                        continue;
                    }
                    type_name_locations.insert(decl_name.clone(), class_decl_location);
                } else {
                    if let Some(loc) = deprecated_class_definition_locations.get(&decl_name) {
                        let loc = *loc;
                        self.report_error(
                            class_decl_location,
                            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
                                decl_name.clone(),
                                Some(loc),
                            )),
                        );
                        unsafe {
                            (*scope).bindings.insert(
                                Symbol::from_local(name_local),
                                make_binding(error_type, class_decl_location),
                            );
                            *(*scope).lvalue_types.get_or_insert(the_def) = error_type;
                        }
                        continue;
                    }
                    deprecated_class_definition_locations
                        .insert(decl_name.clone(), class_decl_location);
                }

                let the_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };
                unsafe {
                    (*scope).bindings.insert(
                        Symbol::from_local(name_local),
                        make_binding(the_ty, name_local_location),
                    );
                    *(*scope).lvalue_types.get_or_insert(the_def) = the_ty;
                }

                let any_type = unsafe { (*self.builtin_types).anyType };

                // Objects are ExternTypes (see C++ comment block).
                let mut static_props: BTreeMap<Name, Property> = BTreeMap::new();
                let mut props: BTreeMap<Name, Property> = BTreeMap::new();
                let mut instance_metatable_props: BTreeMap<Name, Property> = BTreeMap::new();

                let members_data = unsafe { (*class_decl).members.data };
                let members_size = unsafe { (*class_decl).members.size };
                for m in 0..members_size {
                    let member = unsafe { &*members_data.add(m) };
                    if let Some(class_prop) = member.get_if_0() {
                        // AstClassProperty branch
                        let prop_name = ast_name_to_string(class_prop.name);
                        if props.contains_key(&prop_name) {
                            continue;
                        }

                        let prop_ty = if !class_prop.ty.is_null() {
                            self.resolve_type(scope, class_prop.ty, false, false, Polarity::Unknown)
                        } else {
                            any_type
                        };
                        let mut p = Property::rw_type_id(prop_ty);
                        p.location = Some(class_prop.name_location);
                        props.insert(prop_name, p);
                    } else if let Some(method) = member.get_if_1() {
                        // AstClassMethod branch
                        let method_name = ast_name_to_string(method.function_name);
                        if props.contains_key(&method_name) {
                            continue;
                        }

                        let blocked = unsafe { (*self.arena).add_type(BlockedType::default()) };
                        let mut prop = Property::readonly(blocked);
                        prop.location = Some(method.name_location);

                        let fn_ptr = method.function;
                        let fn_args = unsafe { (*fn_ptr).args };
                        let first_is_self = fn_args.size >= 1
                            && ast_name_is(unsafe { (*(*fn_args.data.add(0))).name }, b"self");
                        if !first_is_self {
                            static_props.insert(method_name.clone(), prop.clone());
                        }
                        // The parser will report an error for classes that define disallowed
                        // metamethods. (See C++ comment.)
                        if is_valid_class_metamethod(&method_name) {
                            instance_metatable_props.insert(method_name.clone(), prop);
                        } else {
                            props.insert(method_name.clone(), prop);
                        }
                    }
                }

                let instance_metatable = unsafe {
                    (*self.arena).add_type(
                        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                            &instance_metatable_props,
                            None,
                            TypeLevel::default(),
                            scope,
                            TableState::Sealed,
                        ),
                    )
                };

                let object_type = unsafe { (*self.builtin_types).objectType };
                let class_type = unsafe { (*self.builtin_types).classType };
                let unknown_type = unsafe { (*self.builtin_types).unknownType };
                let module_name = self.module.as_ref().unwrap().name.clone();

                let class_instance_ty = unsafe {
                    (*self.arena).add_type(make_extern_type(
                        decl_name.clone(),
                        props,
                        Some(object_type),
                        Some(instance_metatable),
                        module_name.clone(),
                        class_decl_location,
                    ))
                };

                let ctor_arg_ty = unsafe {
                    (*self.arena).add_type(
                        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                            &BTreeMap::new(),
                            None,
                            TypeLevel::default(),
                            scope,
                            TableState::Sealed,
                        ),
                    )
                };
                let ctor_arg_table = unsafe { get_mutable_type_id::<TableType>(ctor_arg_ty) };
                debug_assert!(!ctor_arg_table.is_null());
                for m in 0..members_size {
                    let member = unsafe { &*members_data.add(m) };
                    if let Some(prop) = member.get_if_0() {
                        let prop_ty = if !prop.ty.is_null() {
                            self.resolve_type(scope, prop.ty, false, false, Polarity::Unknown)
                        } else {
                            any_type
                        };
                        unsafe {
                            (*ctor_arg_table).props.insert(
                                ast_name_to_string(prop.name),
                                Property::rw_type_id(prop_ty),
                            );
                        }
                    }
                }

                let ctor_ty = unsafe {
                    let arg_pack = self.add_type_pack(alloc::vec![unknown_type, ctor_arg_ty], None);
                    let ret_pack = self.add_type_pack(alloc::vec![class_instance_ty], None);
                    (*self.arena).add_type(FunctionType::function_type_new(
                        arg_pack, ret_pack, None, false,
                    ))
                };

                let mut metatable_props: BTreeMap<Name, Property> = BTreeMap::new();
                metatable_props.insert(String::from("__call"), Property::readonly(ctor_ty));
                let metatable_ty = unsafe {
                    (*self.arena).add_type(
                        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                            &metatable_props,
                            None,
                            TypeLevel::default(),
                            scope,
                            TableState::Sealed,
                        ),
                    )
                };

                let extern_ty = unsafe {
                    (*self.arena).add_type(make_extern_type(
                        decl_name.clone(),
                        static_props,
                        Some(class_type),
                        Some(metatable_ty),
                        module_name.clone(),
                        class_decl_location,
                    ))
                };

                // Setup a bidirectional relationship between classes and objects
                unsafe {
                    (*get_mutable_type_id::<ExternType>(extern_ty)).relation =
                        Some(NominalRelation::V0(Obj {
                            ty: class_instance_ty,
                        }));
                    (*get_mutable_type_id::<ExternType>(class_instance_ty)).relation =
                        Some(NominalRelation::V1(Klass { ty: extern_ty }));
                }

                debug_assert!(unsafe {
                    get_type_id::<crate::records::bound_type::BoundType>(the_ty).is_null()
                });
                let bt = unsafe { get_type_id::<BlockedType>(the_ty) };
                debug_assert!(!bt.is_null());
                debug_assert!(unsafe { (*bt).get_owner().is_null() });

                // emplaceType<BoundType>(asMutable(theTy), externTy)
                unsafe {
                    (*as_mutable_type_id(the_ty)).ty =
                        crate::enums::type_variant::TypeVariant::Bound(extern_ty);
                }

                if unsafe { (*class_decl).exported } {
                    unsafe {
                        (*scope).exported_type_bindings.insert(
                            ast_name_to_string((*name_local).name),
                            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                                Vec::new(),
                                class_instance_ty,
                                Some(class_decl_location),
                            ),
                        );
                    }
                } else {
                    unsafe {
                        (*scope).private_type_bindings.insert(
                            ast_name_to_string((*name_local).name),
                            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                                Vec::new(),
                                class_instance_ty,
                                Some(class_decl_location),
                            ),
                        );
                    }
                }

                *self.class_decl_records.get_or_insert(name_local) = ClassDeclRecord {
                    data_decl: class_decl,
                    ty: class_instance_ty,
                };
                continue;
            }
        }

        if has_type_function {
            // typeFunctionEnvScope = std::make_shared<Scope>(typeFunctionRuntime->rootScope);
            let root_scope = unsafe { (*self.type_function_runtime).root_scope.clone() };
            type_function_env_scope = Some(Arc::new(Scope::new(&root_scope, 0)));
        }

        let mut created_type_functions: Vec<*mut TypeFunctionInstanceType> = Vec::new();
        // DenseHashMap<AstStatTypeFunction*, const TypeFunctionInstanceType*> referencedTypeFunctions{nullptr};
        let mut referenced_type_functions: BTreeMap<
            *mut AstStatTypeFunction,
            *const TypeFunctionInstanceType,
        > = BTreeMap::new();

        // Additional pass for user-defined type functions to fill in their environments completely
        for i in 0..body.size {
            let stat = unsafe { *body.data.add(i) };
            let node = stat as *mut AstNode;
            let function = unsafe { (*node).as_item_mut::<AstStatTypeFunction>() };
            if function.is_null() {
                continue;
            }

            let function_name = unsafe { (*function).name };
            let function_name_str = ast_name_to_string(function_name);
            let function_location = unsafe { (*function).base.base.location };

            let env_scope = type_function_env_scope.clone().unwrap();
            let env_scope_raw = env_scope.as_ref() as *const Scope as *mut Scope;

            // Similar to global pre-population, create a binding for each type function in the scope upfront
            let bt = unsafe { (*self.arena).add_type(BlockedType::default()) };
            unsafe {
                (*env_scope_raw).bindings.insert(
                    Symbol::from_global(function_name),
                    make_binding(bt, function_location),
                );
            }
            *self
                .ast_type_function_environment_scopes
                .get_or_insert(function as *const AstStatTypeFunction) = Some(env_scope.clone());

            // Find the type function we have already created
            let mut main_type_fun: *mut TypeFunctionInstanceType = core::ptr::null_mut();

            if let Some(it) = unsafe { (*scope).private_type_bindings.get(&function_name_str) } {
                main_type_fun =
                    unsafe { get_mutable_type_id::<TypeFunctionInstanceType>(it.r#type) };
            }

            if main_type_fun.is_null() {
                if let Some(it) = unsafe { (*scope).exported_type_bindings.get(&function_name_str) }
                {
                    main_type_fun =
                        unsafe { get_mutable_type_id::<TypeFunctionInstanceType>(it.r#type) };
                }
            }

            // Fill it with all visible type functions and referenced type aliases
            if !main_type_fun.is_null() {
                created_type_functions.push(main_type_fun);

                let mut global_name_collector = GlobalNameCollector::new();
                unsafe { luaur_ast::visit::ast_stat_visit(stat, &mut global_name_collector) };

                // Go up the scopes to register type functions and aliases, but without reaching
                // into the global scope.
                let global_scope_raw = global_scope.as_ref() as *const Scope as *mut Scope;
                let mut level: usize = 0;
                let mut curr: *mut Scope = scope;
                while !curr.is_null() && curr != global_scope_raw {
                    // Collect (name, tf) pairs first to avoid aliasing self & curr during mutation.
                    let private_pairs: Vec<(Name, TypeFun)> = unsafe {
                        (*curr)
                            .private_type_bindings
                            .iter()
                            .map(|(n, tf)| (n.clone(), tf.clone()))
                            .collect()
                    };
                    for (name, tf) in private_pairs {
                        self.proto_add_to_environment(
                            main_type_fun,
                            env_scope_raw,
                            &name,
                            tf,
                            level,
                            &global_name_collector,
                            &mut referenced_type_functions,
                        );
                    }

                    let exported_pairs: Vec<(Name, TypeFun)> = unsafe {
                        (*curr)
                            .exported_type_bindings
                            .iter()
                            .map(|(n, tf)| (n.clone(), tf.clone()))
                            .collect()
                    };
                    for (name, tf) in exported_pairs {
                        self.proto_add_to_environment(
                            main_type_fun,
                            env_scope_raw,
                            &name,
                            tf,
                            level,
                            &global_name_collector,
                            &mut referenced_type_functions,
                        );
                    }

                    level += 1;
                    curr = unsafe {
                        match &(*curr).parent {
                            Some(p) => p.as_ref() as *const Scope as *mut Scope,
                            None => core::ptr::null_mut(),
                        }
                    };
                }
            }
        }

        // Finally, we need to include aliases from functions we might call
        for type_ptr in &created_type_functions {
            let type_ptr = *type_ptr;
            // Go over all functions in our environment.
            let environment_function_pairs: Vec<(Name, (*mut AstStatTypeFunction, usize))> = unsafe {
                (*type_ptr)
                    .user_func_data
                    .environment_function
                    .iter()
                    .map(|(n, v)| (n.clone(), *v))
                    .collect()
            };

            for (_target_func_name, definition_and_level) in environment_function_pairs {
                if let Some(it) = referenced_type_functions.get(&definition_and_level.0) {
                    let target = *it;
                    let target_alias_pairs: Vec<(Name, (*mut TypeFun, usize))> = unsafe {
                        (*target)
                            .user_func_data
                            .environment_alias
                            .iter()
                            .map(|(n, v)| (n.clone(), *v))
                            .collect()
                    };
                    for (alias_name, type_and_level) in target_alias_pairs {
                        let already = unsafe {
                            (*type_ptr)
                                .user_func_data
                                .environment_alias
                                .find(&alias_name)
                                .is_some()
                        };
                        if !already {
                            // Combine definition levels because we are viewing target function
                            // aliases from the perspective of the target function.
                            unsafe {
                                *(*type_ptr)
                                    .user_func_data
                                    .environment_alias
                                    .get_or_insert(alias_name.clone()) =
                                    (type_and_level.0, type_and_level.1 + definition_and_level.1);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Port of the local `addToEnvironment` lambda (L1196-1234).
    #[allow(clippy::too_many_arguments)]
    fn proto_add_to_environment(
        &mut self,
        main_type_fun: *mut TypeFunctionInstanceType,
        env_scope_raw: *mut Scope,
        name: &Name,
        tf: TypeFun,
        level: usize,
        global_name_collector: &GlobalNameCollector,
        referenced_type_functions: &mut BTreeMap<
            *mut AstStatTypeFunction,
            *const TypeFunctionInstanceType,
        >,
    ) {
        let followed = unsafe { crate::functions::follow_type::follow(tf.r#type) };
        let ty = unsafe { get_type_id::<TypeFunctionInstanceType>(followed) };

        if !ty.is_null() && !unsafe { (*ty).user_func_data.definition.is_null() } {
            let definition = unsafe { (*ty).user_func_data.definition };
            let user_func_data = unsafe { &mut (*main_type_fun).user_func_data };
            if user_func_data.environment_function.find(name).is_some() {
                return;
            }

            *user_func_data
                .environment_function
                .get_or_insert(name.clone()) = (definition, level);

            referenced_type_functions.insert(definition, ty as *const TypeFunctionInstanceType);

            if let Some(it) = self
                .ast_type_function_environment_scopes
                .find(&(definition as *const AstStatTypeFunction))
                .and_then(|o| o.clone())
            {
                let it_raw = it.as_ref() as *const Scope as *mut Scope;
                if let Some(existing) = unsafe { (*it_raw).linear_search_for_binding(name, false) }
                {
                    let def_name = unsafe { (*definition).name };
                    let def_location = unsafe { (*definition).base.base.location };
                    unsafe {
                        (*env_scope_raw).bindings.insert(
                            Symbol::from_global(def_name),
                            make_binding(existing.type_id, def_location),
                        );
                    }
                }
            }
        } else if unsafe { get_type_id::<TypeFunctionInstanceType>(followed).is_null() } {
            let user_func_data = unsafe { &mut (*main_type_fun).user_func_data };
            if user_func_data.environment_alias.find(name).is_some() {
                return;
            }

            // AstName astName = module->names->get(name.c_str());
            let module = self.module.as_ref().unwrap().clone();
            let c_name = alloc::ffi::CString::new(name.as_str()).unwrap_or_default();
            let ast_name = module.names.as_ref().unwrap().get(c_name.as_ptr());

            // Only register globals that we have detected to be used
            if global_name_collector.names.find(&ast_name).is_none() {
                return;
            }

            // Function evaluation environment needs a stable reference to the alias.
            // module->typeFunctionAliases.push_back(make_unique<TypeFun>(tf));
            let def_loc = tf.definition_location.unwrap_or_else(Location::default);
            let module_raw = module.as_ref() as *const crate::records::module::Module
                as *mut crate::records::module::Module;
            unsafe {
                (*module_raw).type_function_aliases.push(Box::new(tf));
            }
            let back_ptr: *mut TypeFun = unsafe {
                (*module_raw)
                    .type_function_aliases
                    .last_mut()
                    .map(|b| b.as_mut() as *mut TypeFun)
                    .unwrap()
            };

            *user_func_data.environment_alias.get_or_insert(name.clone()) = (back_ptr, level);

            // TODO: create a specific type alias type
            let any_type = unsafe { (*self.builtin_types).anyType };
            unsafe {
                (*env_scope_raw).bindings.insert(
                    Symbol::from_global(ast_name),
                    make_binding(any_type, def_loc),
                );
            }
        }
    }
}

/// Build a `Binding{ty, loc}` (Binding has no constructor in the port).
fn make_binding(type_id: TypeId, location: Location) -> crate::records::binding::Binding {
    crate::records::binding::Binding {
        type_id,
        location,
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
    }
}

/// Build an `ExternType{name, props, parent, metatable, Tags{}, nullptr, moduleName, location}`.
fn make_extern_type(
    name: Name,
    props: BTreeMap<Name, Property>,
    parent: Option<TypeId>,
    metatable: Option<TypeId>,
    definition_module_name: String,
    definition_location: Location,
) -> ExternType {
    ExternType {
        name,
        props,
        parent,
        metatable,
        tags: Vec::new(),
        user_data: None,
        definition_module_name,
        definition_location: Some(definition_location),
        indexer: None,
        relation: None,
    }
}

/// AstName -> owned String (for map keys / names).
fn ast_name_to_string(name: AstName) -> String {
    if name.value.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() }
}

/// AstName == literal (byte comparison of the C string).
fn ast_name_is(name: AstName, literal: &[u8]) -> bool {
    if name.value.is_null() {
        return false;
    }
    unsafe { CStr::from_ptr(name.value).to_bytes() == literal }
}

// Silence unused-import warnings for items only referenced in some FFlag paths.
#[allow(unused_imports)]
use core::ptr as _unused_ptr;
#[allow(unused_imports)]
use AstStat as _UnusedAstStat;
