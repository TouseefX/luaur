use crate::enums::control_flow::ControlFlow;
use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::extern_type::ExternType;
use crate::records::function_argument::FunctionArgument;
use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::intersection_type::IntersectionType;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::unknown_symbol::{Context, UnknownSymbol};
use crate::type_aliases::name_type::Name;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_ast::enums::ast_table_access::AstTableAccess;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use luaur_common::functions::format::format;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_declare_extern_type(
        &mut self,
        scope: *mut Scope,
        declared_extern_type: *mut AstStatDeclareExternType,
    ) -> ControlFlow {
        unsafe {
            let scope_ref: &Scope = &*scope;
            let decl_ref: &AstStatDeclareExternType = &*declared_extern_type;

            // If a class with the same name was already defined, we skip over.
            let name_key: Name = core::ffi::CStr::from_ptr(decl_ref.name.value)
                .to_string_lossy()
                .into_owned();
            let binding_it = match scope_ref.exported_type_bindings.get(&name_key) {
                Some(b) => b.clone(),
                None => return ControlFlow::None,
            };

            let mut super_ty: Option<TypeId> = Some((*self.builtin_types).externType);

            if let Some(super_name_node) = decl_ref.super_name.as_ref() {
                let super_name: Name = core::ffi::CStr::from_ptr(super_name_node.value)
                    .to_string_lossy()
                    .into_owned();

                let lookup_type = scope_ref.lookup_type(&super_name);

                if lookup_type.is_none() {
                    self.report_error(
                        decl_ref.base.base.location,
                        TypeErrorData::UnknownSymbol(UnknownSymbol::new(super_name, Context::Type)),
                    );
                    return ControlFlow::None;
                }

                let lookup_type = lookup_type.unwrap();

                // We don't have generic extern typeArguments, so this assertion
                // _should_ never be hit.
                LUAU_ASSERT!(
                    lookup_type.type_params().len() == 0
                        && lookup_type.type_pack_params().len() == 0
                );

                let followed = follow_type_id(lookup_type.r#type());
                super_ty = Some(followed);

                if get_type_id::<ExternType>(follow_type_id(super_ty.unwrap())).is_null() {
                    self.report_error(
                        decl_ref.base.base.location,
                        TypeErrorData::GenericError(GenericError::new(format(format_args!(
                            "Cannot use non-class type '{}' as a superclass of class '{}'",
                            super_name.as_str(),
                            name_key.as_str()
                        )))),
                    );

                    // If we don't emplace an error type here, then later we'll be
                    // exposing a blocked type in this file's type interface. This
                    // is _normally_ harmless.
                    let class_bind_ty = binding_it.r#type();
                    (*as_mutable_type_id(class_bind_ty)).ty =
                        TypeVariant::Bound((*self.builtin_types).errorType);

                    return ControlFlow::None;
                }
            }

            let class_name: Name = name_key.clone();

            let module_name = self.module.as_ref().unwrap().name.clone();

            let extern_ty: TypeId = (*self.arena).add_type(ExternType {
                name: class_name,
                props: Default::default(),
                parent: super_ty,
                metatable: None,
                tags: Default::default(),
                user_data: None,
                definition_module_name: module_name,
                definition_location: Some(decl_ref.base.base.location),
                indexer: None,
                relation: None,
            });
            let etv = get_mutable_type_id::<ExternType>(extern_ty);

            let meta_ty: TypeId =
                (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
                    TableState::Sealed,
                    scope_ref.level,
                    scope,
                ));
            let metatable = get_mutable_type_id::<TableType>(meta_ty);

            (*etv).metatable = Some(meta_ty);

            let class_bind_ty = binding_it.r#type();
            (*as_mutable_type_id(class_bind_ty)).ty = TypeVariant::Bound(extern_ty);

            if !decl_ref.indexer.is_null() {
                let indexer: &luaur_ast::records::ast_table_indexer::AstTableIndexer =
                    &*decl_ref.indexer;
                if self.recursion_count
                    >= luaur_common::DFInt::LuauConstraintGeneratorRecursionLimit.get() as i32
                {
                    self.report_code_too_complex(indexer.location);
                } else {
                    // I don't think extern types can *be* generic, but if they
                    // have an indexer over those generics, the polarity is
                    // mixed.
                    let index_type =
                        self.resolve_type(scope, indexer.index_type, false, false, Polarity::Mixed);
                    let index_result_type = self.resolve_type(
                        scope,
                        indexer.result_type,
                        false,
                        false,
                        Polarity::Mixed,
                    );
                    (*etv).indexer = Some(TableIndexer {
                        index_type,
                        index_result_type,
                        is_read_only: false,
                    });
                }
            }

            let mut prop_i = 0;
            while prop_i < decl_ref.props.size {
                let extern_prop = *decl_ref.props.data.add(prop_i);
                prop_i += 1;

                let prop_name: Name = core::ffi::CStr::from_ptr(extern_prop.name.value)
                    .to_string_lossy()
                    .into_owned();
                let prop_ty =
                    self.resolve_type(scope, extern_prop.ty, false, false, Polarity::Mixed);

                let assign_to_metatable = is_metamethod_mut(&prop_name);

                // Function typeArguments always take 'self', but this isn't
                // reflected in the parsed annotation. Add it here.
                if extern_prop.is_method {
                    let ftv_ptr = get_mutable_type_id::<FunctionType>(prop_ty);
                    if !ftv_ptr.is_null() {
                        let ftv = &mut *ftv_ptr;
                        ftv.arg_names.insert(
                            0,
                            Some(FunctionArgument {
                                name: "self".into(),
                                location: Default::default(),
                            }),
                        );
                        ftv.arg_types =
                            self.add_type_pack(alloc::vec![extern_ty], Some(ftv.arg_types));

                        ftv.has_self = true;

                        let defn = FunctionDefinition {
                            definition_module_name: Some(
                                self.module.as_ref().unwrap().name.clone(),
                            ),
                            definition_location: extern_prop.location,
                            // No data is preserved for varargLocation
                            vararg_location: None,
                            original_name_location: extern_prop.name_location,
                        };

                        ftv.definition = Some(defn);
                    }
                }

                let props: &mut Props = if assign_to_metatable {
                    &mut (*metatable).props
                } else {
                    &mut (*etv).props
                };

                if !props.contains_key(&prop_name) {
                    let mut table_prop = if extern_prop.access == AstTableAccess::Read {
                        Property::readonly(prop_ty)
                    } else if extern_prop.access == AstTableAccess::Write {
                        Property::writeonly(prop_ty)
                    } else {
                        Property::rw_type_id(prop_ty)
                    };

                    table_prop.location = Some(extern_prop.location);

                    props.insert(prop_name.clone(), table_prop);
                } else {
                    let prop = props.get_mut(&prop_name).unwrap();
                    let mut added_write_type_by_overload = false;

                    if let Some(read_ty) = prop.read_ty {
                        // We special-case this logic to keep the intersection
                        // flat; otherwise we would create a ton of nested
                        // intersection typeArguments.
                        let itv = get_type_id::<IntersectionType>(read_ty);
                        if !itv.is_null() {
                            let mut options = (*itv).parts.clone();
                            options.push(prop_ty);
                            let new_itv =
                                (*self.arena).add_type(IntersectionType { parts: options });
                            prop.read_ty = Some(new_itv);
                        } else if !get_type_id::<FunctionType>(read_ty).is_null() {
                            let intersection = (*self.arena).add_type(IntersectionType {
                                parts: alloc::vec![read_ty, prop_ty],
                            });
                            prop.read_ty = Some(intersection);
                        } else if extern_prop.access == AstTableAccess::Write
                            && prop.write_ty.is_none()
                        {
                            prop.write_ty = Some(prop_ty);
                            added_write_type_by_overload = true;
                        } else {
                            self.report_error(
                                decl_ref.base.base.location,
                                TypeErrorData::GenericError(GenericError::new(format(
                                    format_args!(
                                        "Cannot overload read type of non-function extern type member '{}'",
                                        prop_name.as_str()
                                    ),
                                ))),
                            );
                        }
                    }

                    if let Some(write_ty) = prop.write_ty {
                        if !added_write_type_by_overload {
                            // We special-case this logic to keep the
                            // intersection flat; otherwise we would create a ton
                            // of nested intersection typeArguments.
                            let itv = get_type_id::<IntersectionType>(write_ty);
                            if !itv.is_null() {
                                let mut options = (*itv).parts.clone();
                                options.push(prop_ty);
                                let new_itv =
                                    (*self.arena).add_type(IntersectionType { parts: options });
                                prop.write_ty = Some(new_itv);
                            } else if !get_type_id::<FunctionType>(write_ty).is_null() {
                                let intersection = (*self.arena).add_type(IntersectionType {
                                    parts: alloc::vec![write_ty, prop_ty],
                                });
                                prop.write_ty = Some(intersection);
                            } else if extern_prop.access == AstTableAccess::Read
                                && prop.read_ty.is_none()
                            {
                                prop.read_ty = Some(prop_ty);
                            } else {
                                self.report_error(
                                    decl_ref.base.base.location,
                                    TypeErrorData::GenericError(GenericError::new(format(
                                        format_args!(
                                            "Cannot overload write type of non-function extern type member '{}'",
                                            prop_name.as_str()
                                        ),
                                    ))),
                                );
                            }
                        }
                    }
                }
            }

            ControlFlow::None
        }
    }
}

use crate::functions::is_metamethod_constraint_generator::is_metamethod_mut;
