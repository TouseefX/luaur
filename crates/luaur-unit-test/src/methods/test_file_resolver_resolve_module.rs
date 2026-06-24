use crate::records::test_file_resolver::TestFileResolver;
use luaur_analysis::records::module_info::ModuleInfo;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use std::ffi::CStr;

impl TestFileResolver {
    pub fn resolve_module(
        &self,
        context: Option<&ModuleInfo>,
        expr: &AstExpr,
        _limits: &TypeCheckLimits,
    ) -> Option<ModuleInfo> {
        let node = expr as *const AstExpr as *mut AstNode;

        unsafe {
            if let Some(global) = ast_node_as::<AstExprGlobal>(node).as_ref() {
                let name = ast_name_to_string(global.name.value);

                if name == "game" {
                    return Some(ModuleInfo {
                        name: "game".to_string(),
                        optional: false,
                    });
                }

                if name == "workspace" {
                    return Some(ModuleInfo {
                        name: "workspace".to_string(),
                        optional: false,
                    });
                }

                if name == "script" {
                    return context.cloned();
                }
            } else if let Some(index) = ast_node_as::<AstExprIndexName>(node).as_ref() {
                let context = context?;
                let index_name = ast_name_to_string(index.index.value);

                if index_name == "Parent" {
                    let last_separator = context.name.rfind('/')?;
                    return Some(ModuleInfo {
                        name: context.name[..last_separator].to_string(),
                        optional: context.optional,
                    });
                }

                return Some(ModuleInfo {
                    name: format!("{}/{}", context.name, index_name),
                    optional: context.optional,
                });
            } else if let Some(index) = ast_node_as::<AstExprIndexExpr>(node).as_ref() {
                let context = context?;
                let index_expr = index.index as *mut AstNode;

                if let Some(index_string) =
                    ast_node_as::<AstExprConstantString>(index_expr).as_ref()
                {
                    return Some(ModuleInfo {
                        name: format!(
                            "{}/{}",
                            context.name,
                            constant_string_to_string(index_string)
                        ),
                        optional: context.optional,
                    });
                }
            } else if let Some(call) = ast_node_as::<AstExprCall>(node).as_ref() {
                let context = context?;

                if call.self_ && call.args.size >= 1 && context.name == "game" {
                    let arg = *call.args.data;
                    let arg_node = arg as *mut AstNode;
                    let func_node = call.func as *mut AstNode;

                    if let (Some(index_string), Some(func)) = (
                        ast_node_as::<AstExprConstantString>(arg_node).as_ref(),
                        ast_node_as::<AstExprIndexName>(func_node).as_ref(),
                    ) {
                        if ast_name_to_string(func.index.value) == "GetService" {
                            return Some(ModuleInfo {
                                name: format!("game/{}", constant_string_to_string(index_string)),
                                optional: false,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

fn ast_name_to_string(name: *const core::ffi::c_char) -> String {
    if name.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() }
    }
}

fn constant_string_to_string(expr: &AstExprConstantString) -> String {
    expr.value
        .as_slice()
        .iter()
        .map(|&c| c as u8 as char)
        .collect()
}
