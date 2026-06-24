//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:990-1074`
//!
//! `void deserializeChildren(TypeFunctionFunctionType* f2, FunctionType* f1)`.
//! Introduces the function's generic parameters into the deserializer scope
//! (resolving by name, rejecting packs-in-type-position and duplicates), then
//! shallow-deserializes the generics, generic packs, arg/ret packs and argNames.
use crate::enums::polarity::Polarity;
use crate::functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::records::function_argument::FunctionArgument;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::r#type::Type;
use crate::records::serialized_function_scope::SerializedFunctionScope;
use crate::records::serialized_generic::SerializedGeneric;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack_var::TypePackVar;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_function_type_function_type(
        &mut self,
        f2: *mut TypeFunctionFunctionType,
        f1: *mut FunctionType,
    ) {
        unsafe {
            self.function_scopes.push(SerializedFunctionScope {
                old_queue_size: self.queue.len(),
                function: f2,
            });
            let mut generic_names: BTreeSet<(bool, String)> = BTreeSet::new();

            let arena = (*(*self.state).ctx).arena.as_ptr();
            let scope = (*(*self.state).ctx).scope.as_ptr();

            // Introduce generic function parameters into scope
            for &ty in &(*f2).generics {
                let gty = get_type_function_type_id::<TypeFunctionGenericType>(ty);
                if gty.is_null() || (*gty).isPack() {
                    self.push_runtime_error("Encountered unexpected generic".to_string());
                    return;
                } else {
                    LUAU_ASSERT!(!gty.is_null() && !(*gty).isPack());
                }

                let name_key = ((*gty).isNamed(), (*gty).name().to_string());

                // Duplicates are not allowed
                if generic_names.contains(&name_key) {
                    self.push_runtime_error(format!(
                        "Duplicate type parameter '{}'",
                        (*gty).name()
                    ));
                    return;
                }

                generic_names.insert(name_key);

                let mapping = if (*gty).isNamed() {
                    (*arena).add_tv(Type::from(GenericType::generic_type_scope_name(
                        scope,
                        &(*gty).name().to_string(),
                    )))
                } else {
                    (*arena).add_tv(Type::from(GenericType::generic_type()))
                };
                self.generic_types.push(SerializedGeneric {
                    is_named: (*gty).isNamed(),
                    name: (*gty).name().to_string(),
                    r#type: mapping,
                });
            }

            for &tp in &(*f2).generic_packs {
                let gtp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
                if gtp.is_null() {
                    self.push_runtime_error("Encountered unexpected generic type pack".to_string());
                    return;
                } else {
                    LUAU_ASSERT!(!gtp.is_null());
                }

                let name_key = ((*gtp).isNamed(), (*gtp).name().to_string());

                // Duplicates are not allowed
                if generic_names.contains(&name_key) {
                    self.push_runtime_error(format!(
                        "Duplicate type parameter '{}'",
                        (*gtp).name()
                    ));
                    return;
                }

                generic_names.insert(name_key);

                let mapping = if (*gtp).isNamed() {
                    let gen = GenericTypePack {
                        index: 0,
                        level: TypeLevel::default(),
                        scope,
                        name: (*gtp).name().to_string(),
                        explicitName: true,
                        polarity: Polarity::Unknown,
                    };
                    (*arena).add_type_pack_t(TypePackVar::from(gen))
                } else {
                    let mut gen = GenericTypePack {
                        index: 0,
                        level: TypeLevel::default(),
                        scope: core::ptr::null_mut(),
                        name: String::new(),
                        explicitName: false,
                        polarity: Polarity::Unknown,
                    };
                    gen.generic_type_pack();
                    (*arena).add_type_pack_t(TypePackVar::from(gen))
                };
                self.generic_packs.push(SerializedGeneric {
                    is_named: (*gtp).isNamed(),
                    name: (*gtp).name().to_string(),
                    r#type: mapping,
                });
            }

            (*f1).generics.reserve((*f2).generics.len());
            for &ty in &(*f2).generics {
                let g = self.shallow_deserialize_type_function_type_id(ty);
                (*f1).generics.push(g);
            }

            (*f1).generic_packs.reserve((*f2).generic_packs.len());
            for &tp in &(*f2).generic_packs {
                let g = self.shallow_deserialize_type_function_type_pack_id(tp);
                (*f1).generic_packs.push(g);
            }

            if !(*f2).arg_types.is_null() {
                (*f1).arg_types =
                    self.shallow_deserialize_type_function_type_pack_id((*f2).arg_types);
            }

            if !(*f2).ret_types.is_null() {
                (*f1).ret_types =
                    self.shallow_deserialize_type_function_type_pack_id((*f2).ret_types);
            }

            if luaur_common::FFlag::LuauTypeFunctionSerializeArgNames.get() {
                (*f1).arg_names.reserve((*f2).arg_names.len());
                for name in &(*f2).arg_names {
                    if let Some(name) = name {
                        (*f1).arg_names.push(Some(FunctionArgument {
                            name: name.clone(),
                            location: Location::default(),
                        }));
                    } else {
                        (*f1).arg_names.push(None);
                    }
                }
            }
        }
    }
}
