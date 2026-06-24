use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use alloc::ffi::CString;
use alloc::string::String;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl LintDeprecatedApi {
    pub fn check_ast_expr_index_name_type_id(&mut self, node: *mut AstExprIndexName, ty: TypeId) {
        unsafe {
            let ty = follow_type_id(ty);
            let index = core::ffi::CStr::from_ptr((*node).index.value)
                .to_string_lossy()
                .into_owned();

            let extern_type = get_type_id::<ExternType>(ty);
            if !extern_type.is_null() {
                let prop = lookup_extern_type_prop(&*extern_type, &index);
                if let Some(prop) = prop.as_ref() {
                    if prop.deprecated {
                        let container = CString::new((*extern_type).name.as_str()).unwrap();
                        self.report_location_property_c_char_c_char(
                            &(*node).base.base.location,
                            prop,
                            container.as_ptr(),
                            (*node).index.value,
                        );
                    } else if let Some(read_ty) = prop.read_ty {
                        let fty = get_type_id::<FunctionType>(follow_type_id(read_ty));
                        if !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty) {
                            let global = ast_node_as::<AstExprGlobal>((*node).expr as *mut AstNode);
                            let container = if let Some(global) = global.as_ref() {
                                Some(
                                    CString::new(
                                        core::ffi::CStr::from_ptr(global.name.value)
                                            .to_string_lossy()
                                            .as_ref(),
                                    )
                                    .unwrap(),
                                )
                            } else {
                                None
                            };
                            if let Some(info) = (*fty).deprecated_info.as_deref() {
                                self.report_location_c_char_c_char_ast_attr_deprecated_info(
                                    &(*node).base.base.location,
                                    container
                                        .as_ref()
                                        .map_or(core::ptr::null(), |value| value.as_ptr()),
                                    (*node).index.value,
                                    info,
                                );
                            } else {
                                self.report_location_c_char_c_char(
                                    &(*node).base.base.location,
                                    container
                                        .as_ref()
                                        .map_or(core::ptr::null(), |value| value.as_ptr()),
                                    (*node).index.value,
                                );
                            }
                        }
                    }
                }

                return;
            }

            let table_type = get_type_id::<TableType>(ty);
            if table_type.is_null() {
                return;
            }

            let table = &*table_type;
            let Some(prop) = table.props.get(&index) else {
                return;
            };

            if prop.deprecated {
                let container_name = table.name.as_ref().map(|name| {
                    if name.starts_with("typeof(") && name.ends_with(')') {
                        name[7..name.len() - 1].to_string()
                    } else {
                        name.clone()
                    }
                });

                let container = container_name
                    .as_ref()
                    .map(|name| CString::new(name.as_str()).unwrap());
                self.report_location_property_c_char_c_char(
                    &(*node).base.base.location,
                    prop,
                    container
                        .as_ref()
                        .map_or(core::ptr::null(), |value| value.as_ptr()),
                    (*node).index.value,
                );
            } else if let Some(read_ty) = prop.read_ty {
                let fty = get_type_id::<FunctionType>(follow_type_id(read_ty));
                if !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty) {
                    let global = ast_node_as::<AstExprGlobal>((*node).expr as *mut AstNode);
                    let container = if let Some(global) = global.as_ref() {
                        Some(
                            CString::new(
                                core::ffi::CStr::from_ptr(global.name.value)
                                    .to_string_lossy()
                                    .as_ref(),
                            )
                            .unwrap(),
                        )
                    } else {
                        None
                    };
                    if let Some(info) = (*fty).deprecated_info.as_deref() {
                        self.report_location_c_char_c_char_ast_attr_deprecated_info(
                            &(*node).base.base.location,
                            container
                                .as_ref()
                                .map_or(core::ptr::null(), |value| value.as_ptr()),
                            (*node).index.value,
                            info,
                        );
                    } else {
                        self.report_location_c_char_c_char(
                            &(*node).base.base.location,
                            container
                                .as_ref()
                                .map_or(core::ptr::null(), |value| value.as_ptr()),
                            (*node).index.value,
                        );
                    }
                }
            }
        }
    }
}
