use crate::enums::table_state::TableState;
use crate::enums::type_type_function_runtime::Type as TypeFunctionPrimitiveKind;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::r#type::Type;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_function_any_type::TypeFunctionAnyType;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::tags::Tags;
use crate::type_aliases::type_function_kind::TypeFunctionKind;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use alloc::format;
use alloc::vec::Vec;

impl TypeFunctionDeserializer {
    pub fn shallow_deserialize_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> TypeId {
        if let Some(it) = self.find_type_function_type_id(ty) {
            return it;
        }

        let make_empty_table = || TableType {
            props: Props::default(),
            indexer: None,
            state: TableState::Sealed,
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            name: None,
            synthetic_name: None,
            instantiated_type_params: Vec::new(),
            instantiated_type_pack_params: Vec::new(),
            definition_module_name: Default::default(),
            definition_location: Default::default(),
            bound_to: None,
            tags: Tags::default(),
            remaining_props: 0,
        };

        unsafe {
            let ctx = &mut *(*self.state).ctx;
            let arena = ctx.arena.as_ptr();
            let builtins = ctx.builtins.as_ptr();
            let mut target: TypeId = core::ptr::null();

            if let Some(p) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
                target = match p.r#type {
                    TypeFunctionPrimitiveKind::NilType => (*builtins).nilType,
                    TypeFunctionPrimitiveKind::Boolean => (*builtins).booleanType,
                    TypeFunctionPrimitiveKind::Number => (*builtins).numberType,
                    TypeFunctionPrimitiveKind::Integer => (*builtins).integerType,
                    TypeFunctionPrimitiveKind::String => (*builtins).stringType,
                    TypeFunctionPrimitiveKind::Thread => (*builtins).threadType,
                    TypeFunctionPrimitiveKind::Buffer => (*builtins).bufferType,
                };
            } else if !get_type_function_type_id::<TypeFunctionUnknownType>(ty).is_null() {
                target = (*builtins).unknownType;
            } else if !get_type_function_type_id::<TypeFunctionNeverType>(ty).is_null() {
                target = (*builtins).neverType;
            } else if !get_type_function_type_id::<TypeFunctionAnyType>(ty).is_null() {
                target = (*builtins).anyType;
            } else if let Some(s) =
                get_type_function_type_id::<TypeFunctionSingletonType>(ty).as_ref()
            {
                if let Some(bs) = s.variant.get_if_0() {
                    target = (*arena).add_type(SingletonType {
                        variant: SingletonVariant::V0(BooleanSingleton { value: bs.value }),
                    });
                } else if let Some(ss) = s.variant.get_if_1() {
                    target = (*arena).add_type(SingletonType {
                        variant: SingletonVariant::V1(StringSingleton {
                            value: ss.value.clone(),
                        }),
                    });
                } else {
                    (*ctx.ice.as_ptr()).ice_string(
                        "Deserializing user defined type function arguments: mysterious type is being deserialized",
                    );
                    return core::ptr::null();
                }
            } else if !get_type_function_type_id::<TypeFunctionUnionType>(ty).is_null() {
                target = (*arena).add_tv(Type::from(UnionType {
                    options: Vec::new(),
                }));
            } else if !get_type_function_type_id::<TypeFunctionIntersectionType>(ty).is_null() {
                target = (*arena).add_tv(Type::from(IntersectionType { parts: Vec::new() }));
            } else if !get_type_function_type_id::<TypeFunctionNegationType>(ty).is_null() {
                target = (*arena).add_type(NegationType::new((*builtins).unknownType));
            } else if let Some(table) =
                get_type_function_type_id::<TypeFunctionTableType>(ty).as_ref()
            {
                if table.metatable.is_none() {
                    target = (*arena).add_type(make_empty_table());
                } else {
                    let empty_table = (*arena).add_type(make_empty_table());
                    target = (*arena).add_type(MetatableType::new(empty_table, empty_table));
                }
            } else if !get_type_function_type_id::<TypeFunctionFunctionType>(ty).is_null() {
                let empty_type_pack = (*arena).add_type_pack_t(TypePack::new(Vec::new(), None));
                target = (*arena).add_type(FunctionType::function_type_new(
                    empty_type_pack,
                    empty_type_pack,
                    None,
                    false,
                ));
            } else if let Some(c) = get_type_function_type_id::<TypeFunctionExternType>(ty).as_ref()
            {
                target = c.extern_ty;
            } else if let Some(g) =
                get_type_function_type_id::<TypeFunctionGenericType>(ty).as_ref()
            {
                if g.isPack() {
                    self.push_runtime_error(format!(
                        "Generic type pack '{}...' cannot be placed in a type position",
                        g.name()
                    ));
                    return core::ptr::null();
                }

                if let Some(mapping) = self
                    .generic_types
                    .iter()
                    .rev()
                    .find(|el| el.is_named == g.isNamed() && el.name == g.name())
                    .map(|el| el.r#type)
                {
                    target = mapping;
                } else {
                    self.push_runtime_error(format!(
                        "Generic type '{}' is not in a scope of the active generic function",
                        g.name()
                    ));
                    return core::ptr::null();
                }
            } else {
                (*ctx.ice.as_ptr()).ice_string(
                    "Deserializing user defined type function arguments: mysterious type is being deserialized",
                );
                return core::ptr::null();
            }

            *self.types.get_or_insert(ty) = target;
            self.queue
                .push((TypeFunctionKind::V0(ty), TypeOrPack::V0(target)));
            target
        }
    }
}
