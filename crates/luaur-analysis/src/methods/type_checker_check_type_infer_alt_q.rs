use crate::enums::control_flow::ControlFlow;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_metamethod_type_infer::is_metamethod;
use crate::records::extern_type::ExternType;
use crate::records::function_argument::FunctionArgument;
use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::intersection_type::IntersectionType;
use crate::records::property_type::Property;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_fun::TypeFun;
use crate::records::type_pack::TypePack;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use core::ffi::CStr;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_declare_extern_type(
        &mut self,
        scope: &ScopePtr,
        declared_extern_type: &AstStatDeclareExternType,
    ) -> ControlFlow {
        let class_name: Name = unsafe {
            CStr::from_ptr(declared_extern_type.name.value)
                .to_string_lossy()
                .into_owned()
        };

        if self
            .incorrect_extern_type_definitions
            .contains(&(declared_extern_type as *const AstStatDeclareExternType))
        {
            return ControlFlow::None;
        }

        let binding: Option<TypeFun> = scope.exported_type_bindings.get(&class_name).cloned();
        if binding.is_none() {
            self.ice_string("Extern type not predeclared");
            return ControlFlow::None;
        }

        let extern_ty = binding.unwrap().r#type;
        let etv = unsafe { get_mutable_type_id::<ExternType>(extern_ty) };
        if etv.is_null() {
            self.ice_string("Extern type binding was not an extern type");
            return ControlFlow::None;
        }

        let metatable_ty = unsafe { (*etv).metatable };
        if metatable_ty.is_none() {
            self.ice_string("No metatable for declared extern type");
            return ControlFlow::None;
        }

        if !declared_extern_type.indexer.is_null() {
            let indexer = unsafe { &*declared_extern_type.indexer };
            let index_type = self.resolve_type(scope.clone(), unsafe { &*indexer.index_type });
            let result_type = self.resolve_type(scope.clone(), unsafe { &*indexer.result_type });
            unsafe {
                (*etv).indexer = Some(TableIndexer {
                    index_type,
                    index_result_type: result_type,
                    is_read_only: indexer.access
                        == luaur_ast::enums::ast_table_access::AstTableAccess::Read,
                });
            }
        }

        let metatable = unsafe { get_mutable_type_id::<TableType>(metatable_ty.unwrap()) };
        if metatable.is_null() {
            self.ice_string("Declared extern type metatable was not a table");
            return ControlFlow::None;
        }

        for prop in declared_extern_type.props.iter() {
            let prop_name: Name = unsafe {
                CStr::from_ptr(prop.name.value)
                    .to_string_lossy()
                    .into_owned()
            };
            let prop_ty = self.resolve_type(scope.clone(), unsafe { &*prop.ty });
            let assign_to_metatable = is_metamethod(&prop_name);

            if prop.is_method {
                let ftv = unsafe { get_mutable_type_id::<FunctionType>(prop_ty) };
                if !ftv.is_null() {
                    unsafe {
                        (*ftv).arg_names.insert(
                            0,
                            Some(FunctionArgument {
                                name: "self".to_string(),
                                location: prop.location,
                            }),
                        );
                        (*ftv).arg_types = self.add_type_pack_type_pack(TypePack {
                            head: alloc::vec::Vec::from([extern_ty]),
                            tail: Some((*ftv).arg_types),
                        });
                        (*ftv).has_self = true;
                        (*ftv).definition = Some(FunctionDefinition {
                            definition_module_name: Some(
                                (*self.current_module.as_ref().unwrap()).name.clone(),
                            ),
                            definition_location: prop.location,
                            vararg_location: None,
                            original_name_location: prop.name_location,
                        });
                    }
                }
            }

            let assign_to = if assign_to_metatable {
                unsafe { &mut (*metatable).props }
            } else {
                unsafe { &mut (*etv).props }
            };

            if !assign_to.contains_key(&prop_name) {
                assign_to.insert(
                    prop_name,
                    Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
                        prop_ty,
                        false,
                        alloc::string::String::new(),
                        Some(prop.location),
                        Default::default(),
                        None,
                        None,
                    ),
                );
            } else {
                let property = assign_to.get_mut(&prop_name).unwrap();
                let current_ty = property.type_deprecated();
                let current_ty = unsafe { follow_type_id(current_ty) };

                let current_intersection = unsafe { get_type_id::<IntersectionType>(current_ty) };
                if !current_intersection.is_null() {
                    let mut options = unsafe { (*current_intersection).parts.clone() };
                    options.push(prop_ty);
                    let new_itv = self.add_type(&IntersectionType { parts: options });
                    property.read_ty = Some(new_itv);
                    property.write_ty = Some(new_itv);
                } else if unsafe { !get_type_id::<FunctionType>(current_ty).is_null() } {
                    let intersection = self.add_type(&IntersectionType {
                        parts: alloc::vec::Vec::from([current_ty, prop_ty]),
                    });
                    property.read_ty = Some(intersection);
                    property.write_ty = Some(intersection);
                } else {
                    self.report_error_location_type_error_data(
                        &declared_extern_type.base.base.location,
                        TypeErrorData::GenericError(GenericError::new(format!(
                            "Cannot overload non-function class member '{}'",
                            prop_name
                        ))),
                    );
                }
            }
        }

        ControlFlow::None
    }
}
