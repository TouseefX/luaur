//! Faithful port of `TypeFunctionSerializer::shallowSerialize(TypeId ty)`
//! (Analysis/src/TypeFunctionRuntimeBuilder.cpp:145-255).
use crate::enums::type_type_function_runtime::Type as TypeFunctionPrimitiveKind;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::{PrimitiveType, Type as PrimitiveKind};
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_function_any_type::TypeFunctionAnyType;
use crate::records::type_function_boolean_singleton::TypeFunctionBooleanSingleton;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_string_singleton::TypeFunctionStringSingleton;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::records::type_function_type::TypeFunctionType;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::records::unsupported_type::UnsupportedType;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorData;
use crate::type_aliases::type_function_kind::TypeFunctionKind;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

impl TypeFunctionSerializer {
    pub fn shallow_serialize_type_id(&mut self, ty: TypeId) -> TypeFunctionTypeId {
        let ty = unsafe { follow_type_id(ty) };

        // if (auto it = find(ty)) return *it;
        if let Some(it) = self.types.get(&ty).copied() {
            return it;
        }

        let state = self.state;
        let runtime = self.type_function_runtime;

        // Create a shallow serialization
        let mut target: TypeFunctionTypeId = core::ptr::null();

        unsafe {
            if let Some(p) = get_type_id::<PrimitiveType>(ty).as_ref() {
                match p.r#type {
                    PrimitiveKind::NilType => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::NilType,
                            }),
                        ));
                    }
                    PrimitiveKind::Boolean => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::Boolean,
                            }),
                        ));
                    }
                    PrimitiveKind::Number => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::Number,
                            }),
                        ));
                    }
                    PrimitiveKind::Integer => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::Integer,
                            }),
                        ));
                    }
                    PrimitiveKind::String => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::String,
                            }),
                        ));
                    }
                    PrimitiveKind::Thread => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::Thread,
                            }),
                        ));
                    }
                    PrimitiveKind::Buffer => {
                        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                                r#type: TypeFunctionPrimitiveKind::Buffer,
                            }),
                        ));
                    }
                    // case Function: case Table: default:
                    _ => {
                        if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                            (*state).errors.push(
                                crate::records::type_function_error::TypeFunctionError {
                                    location: Location::default(),
                                    module_name: alloc::string::String::new(),
                                    data: TypeFunctionErrorData::V0(UnsupportedType { r#type: ty }),
                                },
                            );
                        } else {
                            (*state).errors_deprecated.push(format!(
                                "Argument of primitive type {} is not currently serializable by type functions",
                                to_string_type_id(ty)
                            ));
                        }
                    }
                }
            } else if !get_type_id::<UnknownType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Unknown(TypeFunctionUnknownType { _unused: None }),
                ));
            } else if !get_type_id::<NeverType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Never(TypeFunctionNeverType { _unused: None }),
                ));
            } else if !get_type_id::<AnyType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Any(TypeFunctionAnyType { _unused: None }),
                ));
            } else if let Some(s) = get_type_id::<SingletonType>(ty).as_ref() {
                if let Some(bs) = s.variant.get_if::<BooleanSingleton>() {
                    target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
                            variant: luaur_common::records::variant::Variant2::V0(
                                TypeFunctionBooleanSingleton { value: bs.value },
                            ),
                        }),
                    ));
                } else if let Some(ss) = s.variant.get_if::<StringSingleton>() {
                    target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
                            variant: luaur_common::records::variant::Variant2::V1(
                                TypeFunctionStringSingleton {
                                    value: ss.value.clone(),
                                },
                            ),
                        }),
                    ));
                } else {
                    if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                        (*state).errors.push(
                            crate::records::type_function_error::TypeFunctionError {
                                location: Location::default(),
                                module_name: alloc::string::String::new(),
                                data: TypeFunctionErrorData::V0(UnsupportedType { r#type: ty }),
                            },
                        );
                    } else {
                        (*state).errors_deprecated.push(format!(
                            "Argument of singleton type {} is not currently serializable by type functions",
                            to_string_type_id(ty)
                        ));
                    }
                }
            } else if !get_type_id::<UnionType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Union(TypeFunctionUnionType {
                        components: Vec::new(),
                    }),
                ));
            } else if !get_type_id::<IntersectionType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Intersection(TypeFunctionIntersectionType {
                        components: Vec::new(),
                    }),
                ));
            } else if !get_type_id::<NegationType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Negation(TypeFunctionNegationType {
                        type_id: core::ptr::null(),
                    }),
                ));
            } else if !get_type_id::<TableType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Table(TypeFunctionTableType {
                        props: BTreeMap::new(),
                        indexer: None,
                        metatable: None,
                    }),
                ));
            } else if !get_type_id::<MetatableType>(ty).is_null() {
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Table(TypeFunctionTableType {
                        props: BTreeMap::new(),
                        indexer: None,
                        metatable: None,
                    }),
                ));
            } else if !get_type_id::<FunctionType>(ty).is_null() {
                let empty_type_pack: TypeFunctionTypePackId =
                    (*runtime)
                        .type_pack_arena
                        .allocate(TypeFunctionTypePackVar::new(
                            TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
                                head: Vec::new(),
                                tail: None,
                            }),
                        ));
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Function(TypeFunctionFunctionType {
                        generics: Vec::new(),
                        generic_packs: Vec::new(),
                        arg_types: empty_type_pack,
                        ret_types: empty_type_pack,
                        arg_names: Vec::new(),
                    }),
                ));
            } else if !get_type_id::<ExternType>(ty).is_null() {
                // Since there aren't any new class types being created in type functions, we will
                // deserialize by using a direct reference to the original class
                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Extern(TypeFunctionExternType {
                        props: BTreeMap::new(),
                        indexer: None,
                        metatable: None,
                        read_parent: None,
                        write_parent: None,
                        extern_ty: ty,
                    }),
                ));
            } else if let Some(g) = get_type_id::<GenericType>(ty).as_ref() {
                let mut name = g.name.clone();

                if !g.explicit_name {
                    name = format!("g{}", g.index);
                }

                target = (*runtime).type_arena.allocate(TypeFunctionType::new(
                    TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
                        is_named: g.explicit_name,
                        is_pack: false,
                        name,
                    }),
                ));
            } else {
                if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                    (*state)
                        .errors
                        .push(crate::records::type_function_error::TypeFunctionError {
                            location: Location::default(),
                            module_name: alloc::string::String::new(),
                            data: TypeFunctionErrorData::V0(UnsupportedType { r#type: ty }),
                        });
                } else {
                    (*state).errors_deprecated.push(format!(
                        "Argument of type {} is not currently serializable by type functions",
                        to_string_type_id(ty)
                    ));
                }
            }
        }

        // types[ty] = target;
        *self.types.get_or_insert(ty) = target;
        // queue.emplace_back(ty, target);
        self.queue
            .push((TypeOrPack::V0(ty), TypeFunctionKind::V0(target)));
        target
    }
}
