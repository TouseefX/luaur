//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Method:Luau.UnitTest:tests/Frontend.test.cpp:30:naive_file_resolver_resolve_module`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//!   - type_ref <- method TestFileResolver::resolveModule (tests/Fixture.cpp)
//! - outgoing:
//!   - type_ref -> record ModuleInfo (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item NaiveFileResolver::resolveModule

use crate::records::naive_file_resolver::NaiveFileResolver;
use alloc::format;
use alloc::string::String;
use luaur_analysis::records::file_resolver::FileResolver;
use luaur_analysis::records::module_info::ModuleInfo;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use std::ffi::CStr;

pub unsafe fn naive_file_resolver_resolve_module_vtable(
    this: *mut FileResolver,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
    if expr.is_null() {
        return None;
    }

    let resolver = this as *const NaiveFileResolver;
    let context = if context.is_null() {
        None
    } else {
        Some(unsafe { &*context })
    };

    unsafe { (*resolver).resolve_module(context, &*expr, limits) }
}

impl NaiveFileResolver {
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

                if name == "Modules" {
                    return Some(ModuleInfo {
                        name: String::from("Modules"),
                        optional: false,
                    });
                }

                if name == "game" {
                    return Some(ModuleInfo {
                        name: String::from("game"),
                        optional: false,
                    });
                }
            } else if let Some(index) = ast_node_as::<AstExprIndexName>(node).as_ref() {
                let context = context?;
                return Some(ModuleInfo {
                    name: format!("{}/{}", context.name, ast_name_to_string(index.index.value)),
                    optional: context.optional,
                });
            } else if let Some(call) = ast_node_as::<AstExprCall>(node).as_ref() {
                let context = context?;

                if call.self_ && call.args.size >= 1 {
                    let arg = *call.args.data;
                    let arg_node = arg as *mut AstNode;
                    let func_node = call.func as *mut AstNode;

                    if let (Some(index), Some(func)) = (
                        ast_node_as::<AstExprConstantString>(arg_node).as_ref(),
                        ast_node_as::<AstExprIndexName>(func_node).as_ref(),
                    ) {
                        if ast_name_to_string(func.index.value) == "GetService"
                            && context.name == "game"
                        {
                            return Some(ModuleInfo {
                                name: format!("game/{}", constant_string_to_string(index)),
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
